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
        let san = move_to_san(self.game.position(), mv, self.game.legal_moves())?;
        self.game.play_move(mv)?;
        self.san_history.push(san);
        self.draw_offer = None;

        if let Some(clock) = &mut self.clock {
            let timeout = clock.switch(unix_ms);
            if timeout {
                self.game
                    .timeout(self.game.position().side_to_move.opposite());
            }
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
}

// ToDo: message pack serialization + brotli compression (as optional feature)
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
