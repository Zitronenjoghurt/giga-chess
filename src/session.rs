use crate::core::position::Position;
use crate::error::{ChessError, SessionError, SessionResult};
use crate::game::outcome::GameOutcome;
use crate::game::Game;
use crate::notation::san::move_to_san;
use crate::prelude::{ChessMove, Color};
use crate::session::action::SessionAction;
use crate::session::clock::ChessClock;
use crate::session::config::{SessionConfig, StartingPosition, TimeControl};
use crate::session::event::SessionEvent;
use std::str::FromStr;

pub mod action;
pub mod clock;
pub mod config;
pub mod event;

/// A wrapper for chess games that provides a more useful interface for using a game in practice such as
/// - Validating the color that's trying to move
/// - Events
/// - Clock states
/// - Draw offers
/// - PGN metadata\
///   ...
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Session {
    game: Game,
    config: SessionConfig,
    draw_offer: Option<Color>,
    clock: Option<ChessClock>,
    /// History of moves played in SAN
    san_history: Vec<String>,
}

impl Session {
    pub fn from_config(config: &SessionConfig) -> SessionResult<Self> {
        let position = match &config.starting_position {
            StartingPosition::Default => Position::default(),
            StartingPosition::Fen(fen) => {
                Position::from_str(fen).map_err(SessionError::InvalidFen)?
            }
        };
        let game = Game::from_position(position).with_mode(config.mode);

        let clock = match config.time_control {
            TimeControl::Unlimited => None,
            TimeControl::Clock(config) => Some(ChessClock::from_config(&config)),
        };

        Ok(Self {
            game,
            config: config.clone(),
            draw_offer: None,
            clock,
            san_history: vec![],
        })
    }

    pub fn act(
        &mut self,
        color: Color,
        action: SessionAction,
        unix_ms: u64,
    ) -> SessionResult<SessionEvent> {
        if self.game.is_over() {
            return Err(SessionError::GameOver);
        }

        match action {
            SessionAction::Move(mv) => {
                if color != self.game.position().side_to_move {
                    return Err(SessionError::NotMovingColor);
                }
                self.process_move(mv, unix_ms)?;
                Ok(self.outcome_event())
            }
            SessionAction::MoveFromTo {
                from,
                to,
                promotion,
            } => {
                if color != self.game.position().side_to_move {
                    return Err(SessionError::NotMovingColor);
                }
                let mv = self
                    .game
                    .find_move(from, to, promotion)
                    .ok_or(ChessError::IllegalMove)?;
                self.process_move(mv, unix_ms)?;
                Ok(self.outcome_event())
            }
            SessionAction::Resign => {
                self.game.resign(color);
                Ok(self.outcome_event())
            }
            SessionAction::OfferDraw => {
                if self.draw_offer == Some(color) {
                    Err(SessionError::DrawAlreadyOffered)
                } else if self.draw_offer == Some(color.opposite()) {
                    self.game.agree_draw();
                    Ok(self.outcome_event())
                } else {
                    self.draw_offer = Some(color);
                    Ok(SessionEvent::DrawOffered { by: color })
                }
            }
            SessionAction::AcceptDraw => {
                if self.draw_offer == Some(color.opposite()) {
                    self.game.agree_draw();
                    Ok(self.outcome_event())
                } else {
                    Err(SessionError::NoDrawOffer)
                }
            }
            SessionAction::DeclineDraw => {
                if self.draw_offer == Some(color.opposite()) {
                    self.draw_offer = None;
                    Ok(SessionEvent::DrawOfferDeclined { by: color })
                } else {
                    Err(SessionError::NoDrawOffer)
                }
            }
            SessionAction::ClaimDraw => {
                self.game.claim_draw()?;
                Ok(self.outcome_event())
            }
        }
    }

    fn outcome_event(&self) -> SessionEvent {
        if let Some(outcome) = self.game.outcome() {
            SessionEvent::GameOver(outcome)
        } else {
            SessionEvent::BoardUpdate
        }
    }

    fn process_move(&mut self, mv: ChessMove, unix_ms: u64) -> SessionResult<()> {
        let color = self.game.position().side_to_move;
        let san = move_to_san(self.game.position(), mv, self.game.legal_moves())?;
        self.game.play_move(mv)?;
        self.san_history.push(san);
        self.draw_offer = None;

        if let Some(clock) = &mut self.clock
            && clock.switch(unix_ms)
        {
            self.game.timeout(color);
        }

        Ok(())
    }
}

// Accessors
impl Session {
    pub fn game(&self) -> &Game {
        &self.game
    }

    pub fn config(&self) -> &SessionConfig {
        &self.config
    }

    pub fn draw_offer(&self) -> Option<Color> {
        self.draw_offer
    }

    pub fn clock(&self) -> Option<&ChessClock> {
        self.clock.as_ref()
    }

    pub fn san_history(&self) -> &[String] {
        &self.san_history
    }

    pub fn pgn(&self) -> String {
        crate::notation::pgn::session_pgn(self)
    }

    pub fn turn(&self) -> Color {
        self.game.position().side_to_move
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionRecord {
    pub config: SessionConfig,
    pub draw_offer: Option<Color>,
    pub clock: Option<ChessClock>,
    pub moves: Vec<ChessMove>,
    pub outcome: Option<GameOutcome>,
}

impl Session {
    /// Create a minimal record of this session for long-term storage
    pub fn record(&self) -> SessionRecord {
        SessionRecord {
            config: self.config.clone(),
            draw_offer: self.draw_offer,
            clock: self.clock,
            moves: self.game.history().to_vec(),
            outcome: self.game.outcome(),
        }
    }
}

impl SessionRecord {
    pub fn restore(self) -> SessionResult<Session> {
        let mut session = Session::from_config(&self.config)?;
        for mv in self.moves {
            session.process_move(mv, 0)?;
        }
        if let Some(outcome) = self.outcome {
            session.game.force_outcome(outcome);
        }
        session.draw_offer = self.draw_offer;
        session.clock = self.clock;
        Ok(session)
    }
}

#[cfg(test)]
mod tests {
    use crate::game::mode::GameMode;
    use crate::game::outcome::{DecisiveReason, GameOutcome};
    use crate::prelude::*;
    use crate::session::action::SessionAction;
    use crate::session::clock::ChessClockConfig;
    use crate::session::config::{SessionConfig, StartingPosition, TimeControl};

    fn test_config() -> SessionConfig {
        SessionConfig {
            mode: GameMode::Standard,
            starting_position: StartingPosition::Default,
            time_control: TimeControl::Unlimited,
            pgn: Default::default(),
        }
    }

    #[test]
    fn test_record_and_restore_basic_moves() {
        let config = test_config();
        let mut session = Session::from_config(&config).unwrap();

        session
            .act(
                Color::White,
                SessionAction::MoveFromTo {
                    from: E2,
                    to: E4,
                    promotion: None,
                },
                0,
            )
            .unwrap();
        session
            .act(
                Color::Black,
                SessionAction::MoveFromTo {
                    from: E7,
                    to: E5,
                    promotion: None,
                },
                0,
            )
            .unwrap();
        session
            .act(
                Color::White,
                SessionAction::MoveFromTo {
                    from: G1,
                    to: F3,
                    promotion: None,
                },
                0,
            )
            .unwrap();
        session
            .act(
                Color::Black,
                SessionAction::MoveFromTo {
                    from: B8,
                    to: C6,
                    promotion: None,
                },
                0,
            )
            .unwrap();

        let record = session.record();
        let restored = record.restore().unwrap();

        assert_eq!(session.game().position(), restored.game().position());

        assert_eq!(session.san_history(), restored.san_history());
        assert_eq!(restored.san_history(), &["e4", "e5", "Nf3", "Nc6"]);
    }

    #[test]
    fn test_record_and_restore_draw_offer() {
        let config = test_config();
        let mut session = Session::from_config(&config).unwrap();

        session
            .act(
                Color::White,
                SessionAction::MoveFromTo {
                    from: E2,
                    to: E4,
                    promotion: None,
                },
                0,
            )
            .unwrap();

        session
            .act(Color::White, SessionAction::OfferDraw, 0)
            .unwrap();
        assert_eq!(session.draw_offer(), Some(Color::White));

        let record = session.record();
        let restored = record.restore().unwrap();

        assert_eq!(restored.draw_offer(), Some(Color::White));
    }

    #[test]
    fn test_record_and_restore_checkmate_outcome() {
        let config = test_config();
        let mut session = Session::from_config(&config).unwrap();

        session
            .act(
                Color::White,
                SessionAction::MoveFromTo {
                    from: F2,
                    to: F3,
                    promotion: None,
                },
                0,
            )
            .unwrap();
        session
            .act(
                Color::Black,
                SessionAction::MoveFromTo {
                    from: E7,
                    to: E5,
                    promotion: None,
                },
                0,
            )
            .unwrap();
        session
            .act(
                Color::White,
                SessionAction::MoveFromTo {
                    from: G2,
                    to: G4,
                    promotion: None,
                },
                0,
            )
            .unwrap();
        session
            .act(
                Color::Black,
                SessionAction::MoveFromTo {
                    from: D8,
                    to: H4,
                    promotion: None,
                },
                0,
            )
            .unwrap();

        assert!(session.game().is_over());

        let record = session.record();
        let restored = record.restore().unwrap();

        assert!(restored.game().is_over());
        assert_eq!(session.game().outcome(), restored.game().outcome());
        assert_eq!(
            restored.game().outcome(),
            Some(GameOutcome::Decisive {
                winner: Color::Black,
                reason: DecisiveReason::Checkmate
            })
        );
    }

    #[test]
    fn test_record_and_restore_resignation() {
        let config = test_config();
        let mut session = Session::from_config(&config).unwrap();

        session
            .act(
                Color::White,
                SessionAction::MoveFromTo {
                    from: E2,
                    to: E4,
                    promotion: None,
                },
                0,
            )
            .unwrap();

        session.act(Color::Black, SessionAction::Resign, 0).unwrap();

        let record = session.record();
        let restored = record.restore().unwrap();

        assert!(restored.game().is_over());
        assert_eq!(
            restored.game().outcome(),
            Some(GameOutcome::Decisive {
                winner: Color::White,
                reason: DecisiveReason::Resignation
            })
        );
    }

    #[test]
    fn test_record_and_restore_pgn_headers() {
        let mut config = test_config();

        config.pgn.event = Some("World Championship".to_string());
        config.pgn.white = Some("Magnus Carlsen".to_string());
        config.pgn.black = Some("Ian Nepomniachtchi".to_string());
        config
            .pgn
            .extra
            .push(("ECO".to_string(), "C42".to_string()));

        let session = Session::from_config(&config).unwrap();

        let record = session.record();
        let restored = record.restore().unwrap();

        assert_eq!(session.config().pgn, restored.config().pgn);
        assert_eq!(
            restored.config().pgn.event.as_deref(),
            Some("World Championship")
        );
        assert_eq!(restored.config().pgn.extra.len(), 1);
        assert_eq!(session.pgn(), restored.pgn());
    }

    #[test]
    fn test_record_and_restore_clock_state() {
        let mut config = test_config();

        config.time_control = TimeControl::Clock(ChessClockConfig {
            white_ms: 300_000,
            black_ms: 300_000,
            white_inc_ms: 2_000,
            black_inc_ms: 2_000,
        });

        let mut session = Session::from_config(&config).unwrap();

        session
            .act(
                Color::White,
                SessionAction::MoveFromTo {
                    from: E2,
                    to: E4,
                    promotion: None,
                },
                5000,
            )
            .unwrap();

        session
            .act(
                Color::Black,
                SessionAction::MoveFromTo {
                    from: E7,
                    to: E5,
                    promotion: None,
                },
                8000,
            )
            .unwrap();

        let record = session.record();
        let restored = record.restore().unwrap();

        let original_clock = session.clock().expect("Session should have a clock");
        let restored_clock = restored
            .clock()
            .expect("Restored session should have a clock");

        assert_eq!(original_clock, restored_clock);
        assert_eq!(restored_clock.active(), Color::White);
        assert_eq!(
            original_clock.remaining_ms(Color::White, 10000),
            restored_clock.remaining_ms(Color::White, 10000)
        );
        assert_eq!(
            original_clock.remaining_ms(Color::Black, 10000),
            restored_clock.remaining_ms(Color::Black, 10000)
        );
    }
}
