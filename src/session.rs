use crate::error::{SessionError, SessionResult};
use crate::game::Game;
use crate::prelude::Color;
use crate::session::action::SessionAction;
use crate::session::event::SessionEvent;

mod action;
mod event;

/// A wrapper for chess games that provides a more useful interface for using a game in practice such as
/// - Validating the color that's trying to move
/// - Events
/// - Clock states
/// - Draw offers
/// - PGN metadata\
///   ...
#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Session {
    game: Game,
    draw_offer: Option<Color>,
    // ToDo: Clocks
    // ToDo: PGN
}

impl Session {
    pub fn act(&mut self, color: Color, action: SessionAction) -> SessionResult<SessionEvent> {
        if self.game.is_over() {
            return Err(SessionError::GameOver);
        }

        match action {
            SessionAction::Move(mv) => {
                if color != self.game.position().side_to_move {
                    return Err(SessionError::NotMovingColor);
                }
                self.game.play_move(mv)?;
                self.draw_offer = None;
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
                self.game.play_move_from_to(from, to, promotion)?;
                self.draw_offer = None;
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
}

// Accessors
impl Session {
    pub fn game(&self) -> &Game {
        &self.game
    }

    pub fn draw_offer(&self) -> Option<Color> {
        self.draw_offer
    }
}
