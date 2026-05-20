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
    hash_history: Vec<u64>,
    outcome: Option<GameOutcome>,
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
            hash_history: vec![pos.hash],
            outcome: None,
        }
    }

    pub fn from_moves(moves: &[ChessMove]) -> ChessResult<Self> {
        let mut game = Self::default();
        for (i, mv) in moves.iter().enumerate() {
            if game.play_move(*mv).is_err() {
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
        self.hash_history.push(self.pos.hash);
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
            GameState::DrawFivefold => {
                self.outcome = Some(GameOutcome::Draw(DrawReason::FivefoldRepetition));
            }
            GameState::DrawInsufficientMaterial => {
                self.outcome = Some(GameOutcome::Draw(DrawReason::InsufficientMaterial));
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

    pub fn play(&mut self, from: Square, to: Square) -> ChessResult<()> {
        let mv = self
            .find_move(from, to, None)
            .ok_or(ChessError::IllegalMove)?;
        self.play_move(mv)
    }

    pub fn play_promotion(&mut self, from: Square, to: Square, piece: Piece) -> ChessResult<()> {
        let mv = self
            .find_move(from, to, Some(piece))
            .ok_or(ChessError::IllegalMove)?;
        self.play_move(mv)
    }

    pub fn resign(&mut self, color: Color) {
        self.outcome = Some(GameOutcome::Decisive {
            winner: color.opposite(),
            reason: DecisiveReason::Resignation,
        });
    }

    pub fn timeout(&mut self, color: Color) {
        if !self.pos.board.has_sufficient_material(color.opposite()) {
            self.outcome = Some(GameOutcome::Draw(DrawReason::TimeoutVsInsufficient));
        } else {
            self.outcome = Some(GameOutcome::Decisive {
                winner: color.opposite(),
                reason: DecisiveReason::Timeout,
            });
        }
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
        } else if self.repetition_count() >= 5 {
            GameState::DrawFivefold
        } else if self.legal_moves.is_empty() {
            if MoveGenerator::get().is_in_check(&self.pos, self.pos.side_to_move) {
                GameState::Checkmate
            } else {
                GameState::Stalemate
            }
        } else if self.pos.half_moves >= 100 {
            GameState::DrawFiftyMoveClaimable
        } else if self.repetition_count() >= 3 {
            GameState::DrawRepetitionClaimable
        } else if !self.pos.board.has_sufficient_material(Color::White)
            && !self.pos.board.has_sufficient_material(Color::Black)
        {
            GameState::DrawInsufficientMaterial
        } else {
            GameState::Running
        }
    }

    pub fn repetition_count(&self) -> usize {
        let current = self.pos.hash;
        let search_depth = (self.pos.half_moves as usize).min(self.hash_history.len());
        self.hash_history
            .iter()
            .rev()
            .skip(1)
            .take(search_depth)
            .filter(|&&h| h == current)
            .count()
            + 1
    }

    pub fn outcome(&self) -> Option<GameOutcome> {
        self.outcome
    }

    pub fn is_over(&self) -> bool {
        self.outcome.is_some()
    }

    pub fn pretty_grid(&self) -> String {
        self.pos.pretty_grid()
    }
}

#[cfg(test)]
mod tests {
    use crate::core::position::Position;
    use crate::game::outcome::{DecisiveReason, DrawReason, GameOutcome};
    use crate::game::state::GameState;
    use crate::game::Game;
    use crate::prelude::*;
    use std::str::FromStr;

    fn play(game: &mut Game, from: Square, to: Square) {
        let mv = game
            .find_move(from, to, None)
            .unwrap_or_else(|| panic!("No legal move from {} to {}", from, to));
        game.play_move(mv).unwrap();
    }

    fn play_promo(game: &mut Game, from: Square, to: Square, piece: Piece) {
        let mv = game
            .find_move(from, to, Some(piece))
            .unwrap_or_else(|| panic!("No legal promo move from {} to {}", from, to));
        game.play_move(mv).unwrap();
    }

    #[test]
    fn test_en_passant() {
        let mut game = Game::new();
        play(&mut game, C2, C4);
        play(&mut game, A7, A6);
        play(&mut game, C4, C5);
        play(&mut game, D7, D5);
        play(&mut game, C5, D6);

        let board = game.position().board;
        assert_eq!(board.piece_at(C5), None);
        assert_eq!(board.piece_at(D5), None);
        assert_eq!(board.piece_at(D6), Some((Piece::Pawn, Color::White)));
    }

    #[test]
    fn test_promotion_capture() {
        let mut game = Game::new();

        play(&mut game, D2, D4);
        play(&mut game, E7, E5);
        play(&mut game, D4, E5);
        play(&mut game, D7, D6);
        play(&mut game, E5, D6);
        play(&mut game, C7, C6);
        play(&mut game, D6, D7);
        play(&mut game, E8, E7);
        play_promo(&mut game, D7, C8, Piece::Queen);

        let board = game.position().board;
        assert_eq!(board.piece_at(D7), None);
        assert_eq!(board.piece_at(C8), Some((Piece::Queen, Color::White)));
    }

    #[test]
    fn test_queenside_castle() {
        let mut game = Game::new();

        play(&mut game, E2, E4);
        play(&mut game, D7, D5);
        play(&mut game, E4, E5);
        play(&mut game, B8, C6);
        play(&mut game, A2, A3);
        play(&mut game, C8, F5);
        play(&mut game, B2, B3);
        play(&mut game, D8, D6);
        play(&mut game, C2, C3);
        play(&mut game, E8, C8);

        let board = game.position().board;
        assert_eq!(board.piece_at(A8), None);
        assert_eq!(board.piece_at(E8), None);
        assert_eq!(board.piece_at(C8), Some((Piece::King, Color::Black)));
        assert_eq!(board.piece_at(D8), Some((Piece::Rook, Color::Black)));
    }

    #[test]
    fn test_checkmate_fools_mate() {
        let mut game = Game::new();
        play(&mut game, F2, F3);
        play(&mut game, E7, E5);
        play(&mut game, G2, G4);
        play(&mut game, D8, H4);

        assert!(game.is_over());
        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Decisive {
                winner: Color::Black,
                reason: DecisiveReason::Checkmate,
            })
        );
        assert_eq!(game.state(), GameState::Checkmate);
    }

    #[test]
    fn test_checkmate_scholars_mate() {
        let mut game = Game::new();
        play(&mut game, E2, E4);
        play(&mut game, E7, E5);
        play(&mut game, F1, C4);
        play(&mut game, B8, C6);
        play(&mut game, D1, H5);
        play(&mut game, G8, F6);
        play(&mut game, H5, F7);

        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Decisive {
                winner: Color::White,
                reason: DecisiveReason::Checkmate,
            })
        );
    }

    #[test]
    fn test_resignation_white() {
        let mut game = Game::new();
        game.resign(Color::White);

        assert!(game.is_over());
        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Decisive {
                winner: Color::Black,
                reason: DecisiveReason::Resignation,
            })
        );
    }

    #[test]
    fn test_resignation_black() {
        let mut game = Game::new();
        game.resign(Color::Black);

        assert!(game.is_over());
        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Decisive {
                winner: Color::White,
                reason: DecisiveReason::Resignation,
            })
        );
    }

    #[test]
    fn test_timeout_decisive() {
        let mut game = Game::new();
        game.timeout(Color::White);

        assert!(game.is_over());
        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Decisive {
                winner: Color::Black,
                reason: DecisiveReason::Timeout,
            })
        );
    }

    #[test]
    fn test_timeout_vs_insufficient() {
        let pos = Position::from_str("4k3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        let mut game = Game::from_position(pos);
        game.timeout(Color::White);

        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Draw(DrawReason::TimeoutVsInsufficient))
        );
    }

    #[test]
    fn test_stalemate() {
        let pos = Position::from_str("5Q2/8/8/8/8/K7/8/k7 w - - 0 1").unwrap();
        let mut game = Game::from_position(pos);
        play(&mut game, F8, B4);

        assert!(game.is_over());
        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Draw(DrawReason::Stalemate))
        );
    }

    #[test]
    fn test_draw_agreement() {
        let mut game = Game::new();
        game.agree_draw();

        assert!(game.is_over());
        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Draw(DrawReason::Agreement))
        );
    }

    #[test]
    fn test_insufficient_material_k_vs_k() {
        let pos = Position::from_str("4k3/8/8/8/8/8/8/4K3 w - - 0 1").unwrap();
        let game = Game::from_position(pos);
        assert_eq!(game.state(), GameState::DrawInsufficientMaterial);
    }

    #[test]
    fn test_insufficient_material_via_capture() {
        let pos = Position::from_str("4k3/8/8/8/8/8/8/4K2B w - - 0 1").unwrap();
        let game = Game::from_position(pos);
        assert_eq!(game.state(), GameState::DrawInsufficientMaterial);
    }

    #[test]
    fn test_fifty_move_rule_claim() {
        let pos = Position::from_str("4k3/8/8/8/8/8/4R3/4K3 w - - 100 51").unwrap();
        let mut game = Game::from_position(pos);

        assert_eq!(game.state(), GameState::DrawFiftyMoveClaimable);

        game.claim_draw().unwrap();
        assert!(game.is_over());
        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Draw(DrawReason::FiftyMoveRule))
        );
    }

    #[test]
    fn test_fifty_move_rule_claim_rejected_when_not_claimable() {
        let mut game = Game::new();
        assert!(game.claim_draw().is_err());
    }

    #[test]
    fn test_seventy_five_move_auto() {
        let pos = Position::from_str("4k3/8/8/8/8/8/4R3/4K3 w - - 149 75").unwrap();
        let mut game = Game::from_position(pos);
        play(&mut game, E2, E3);

        assert!(game.is_over());
        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Draw(DrawReason::SeventyFiveMoveRule))
        );
    }

    #[test]
    fn test_threefold_repetition_claim() {
        let mut game = Game::new();

        for _ in 0..2 {
            play(&mut game, G1, F3);
            play(&mut game, G8, F6);
            play(&mut game, F3, G1);
            play(&mut game, F6, G8);
        }

        assert_eq!(game.state(), GameState::DrawRepetitionClaimable);
        game.claim_draw().unwrap();
        assert!(game.is_over());
        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Draw(DrawReason::ThreefoldRepetition))
        );
    }

    #[test]
    fn test_fivefold_repetition_auto() {
        let mut game = Game::new();

        for _ in 0..4 {
            play(&mut game, G1, F3);
            play(&mut game, G8, F6);
            play(&mut game, F3, G1);
            play(&mut game, F6, G8);
        }

        assert!(game.is_over());
        assert_eq!(
            game.outcome(),
            Some(GameOutcome::Draw(DrawReason::FivefoldRepetition))
        );
    }

    #[test]
    fn test_no_draw_claimable_returns_error() {
        let mut game = Game::new();
        let err = game.claim_draw();
        assert_eq!(err, Err(crate::error::ChessError::NoDrawClaimable));
    }

    #[test]
    fn test_illegal_move_after_game_over() {
        let mut game = Game::new();
        play(&mut game, F2, F3);
        play(&mut game, E7, E5);
        play(&mut game, G2, G4);
        play(&mut game, D8, H4);

        assert!(game.legal_moves().is_empty());
    }

    #[test]
    fn test_force_outcome() {
        let mut game = Game::new();
        let outcome = GameOutcome::Draw(DrawReason::Agreement);
        game.force_outcome(outcome);

        assert!(game.is_over());
        assert_eq!(game.outcome(), Some(outcome));
    }
}
