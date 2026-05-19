use crate::core::position::Position;
use crate::error::{ChessError, ChessResult};
use crate::game::mode::GameMode;
use crate::game::outcome::{DecisiveReason, DrawReason, GameOutcome};
use crate::game::state::GameState;
use crate::moves::generator::MoveGenerator;
use crate::moves::list::MoveList;
use crate::prelude::{ChessMove, Color, Piece, Square};

pub mod mode;
pub mod outcome;
pub mod state;

/// A chess game that encapsulates the overall game state as well as current legal moves, move history and outcome.
#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Game {
    mode: GameMode,
    pos: Position,
    legal_moves: MoveList,
    history: Vec<ChessMove>,
    outcome: Option<GameOutcome>,
    // ToDo: Threefold repetition
}

impl Default for Game {
    fn default() -> Self {
        Self::from_position(Position::default())
    }
}

impl Game {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_position(pos: Position) -> Self {
        let legal_moves = MoveGenerator::get().generate(&pos);
        Self {
            mode: GameMode::Standard,
            pos,
            legal_moves,
            history: vec![],
            outcome: None,
        }
    }

    pub fn from_moves(moves: &[ChessMove]) -> ChessResult<Self> {
        let mut game = Self::default();
        for (i, mv) in moves.iter().enumerate() {
            if let Err(err) = game.play_move(*mv) {
                return Err(ChessError::IllegalMoveSequence(i));
            }
        }
        Ok(game)
    }

    pub fn with_mode(mut self, mode: GameMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn play_move(&mut self, mv: ChessMove) -> ChessResult<()> {
        if !self.legal_moves.contains(mv) {
            return Err(ChessError::IllegalMove);
        }

        self.pos = self.pos.make_move(mv);
        self.history.push(mv);
        self.legal_moves = MoveGenerator::get().generate(&self.pos);

        match self.state() {
            GameState::Checkmate => {
                self.outcome = Some(GameOutcome::Decisive {
                    winner: self.pos.side_to_move.opposite(),
                    reason: DecisiveReason::Checkmate,
                })
            }
            GameState::Stalemate => {
                self.outcome = Some(GameOutcome::Draw(DrawReason::Stalemate));
            }
            GameState::DrawSeventyFive => {
                self.outcome = Some(GameOutcome::Draw(DrawReason::SeventyFiveMoveRule));
            }
            _ => {}
        }

        Ok(())
    }

    pub fn find_move(
        &self,
        from: Square,
        to: Square,
        promotion: Option<Piece>,
    ) -> Option<ChessMove> {
        self.legal_moves
            .iter()
            .find(|mv| {
                mv.from() == from && mv.to() == to && mv.flags().promotion_piece() == promotion
            })
            .copied()
    }

    pub fn resign(&mut self, color: Color) {
        self.outcome = Some(GameOutcome::Decisive {
            winner: color.opposite(),
            reason: DecisiveReason::Resignation,
        });
    }

    pub fn timeout(&mut self, color: Color) {
        self.outcome = Some(GameOutcome::Decisive {
            winner: color.opposite(),
            reason: DecisiveReason::Timeout,
        });
    }

    pub fn claim_draw(&mut self) -> Result<(), ChessError> {
        match self.state() {
            GameState::DrawFiftyMoveClaimable => {
                self.outcome = Some(GameOutcome::Draw(DrawReason::FiftyMoveRule));
                Ok(())
            }
            GameState::DrawRepetitionClaimable => {
                self.outcome = Some(GameOutcome::Draw(DrawReason::ThreefoldRepetition));
                Ok(())
            }
            _ => Err(ChessError::NoDrawClaimable),
        }
    }

    pub fn agree_draw(&mut self) {
        self.outcome = Some(GameOutcome::Draw(DrawReason::Agreement));
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn force_outcome(&mut self, outcome: GameOutcome) {
        self.outcome = Some(outcome);
    }
}

// Accessors
impl Game {
    pub fn position(&self) -> &Position {
        &self.pos
    }

    pub fn legal_moves(&self) -> &MoveList {
        &self.legal_moves
    }

    pub fn history(&self) -> &[ChessMove] {
        &self.history
    }

    pub fn state(&self) -> GameState {
        if self.pos.half_moves >= 150 {
            GameState::DrawSeventyFive
        } else if self.legal_moves.is_empty() {
            if MoveGenerator::get().is_in_check(&self.pos, self.pos.side_to_move) {
                GameState::Checkmate
            } else {
                GameState::Stalemate
            }
        } else if self.pos.half_moves >= 100 {
            GameState::DrawFiftyMoveClaimable
        } else {
            GameState::Running
        }
    }

    pub fn outcome(&self) -> Option<GameOutcome> {
        self.outcome
    }
}
