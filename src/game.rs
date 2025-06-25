use crate::engine::Engine;
use crate::game::algebraic_notation::parse_move_to_algebraic_notation;
use crate::game::chess_board::ChessBoard;
use crate::game::chess_move::ChessMove;
use crate::game::color::Color;
use crate::game::pgn_metadata::PGNMetadata;
use crate::game::state::GameState;
use crate::game::status::GameStatus;
use crate::prelude::{Piece, Square};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::sync::Arc;

pub mod algebraic_notation;
pub mod bit_board;
pub mod castling_rights;
pub mod chess_board;
pub mod chess_move;
pub mod color;
pub mod pgn_metadata;
pub mod piece;
pub mod square;
pub mod state;
pub mod status;

/// A chess game that encapsulates the overall game state as well as current legal moves, move history and PGN metadata.
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Game {
    state: GameState,
    status: GameStatus,
    legal_moves: HashSet<ChessMove>,
    move_history: Vec<ChessMove>,
    algebraic_history: Vec<String>,
    pgn_metadata: PGNMetadata,
    origin_fen: Option<String>,
}

impl Game {
    pub fn new(engine: &Arc<Engine>, pgn_metadata: PGNMetadata) -> Self {
        let state = GameState::default();
        let (legal_moves, status) = engine.generate_moves(&state);
        Self {
            state,
            status,
            legal_moves: legal_moves.iter().copied().collect(),
            move_history: Vec::new(),
            algebraic_history: Vec::new(),
            pgn_metadata,
            origin_fen: None,
        }
    }

    pub fn play_move(&mut self, engine: &Arc<Engine>, chess_move: ChessMove) -> bool {
        if !self.legal_moves.contains(&chess_move) {
            return false;
        }

        if self.origin_fen.is_none() {
            if let Some(algebraic) = parse_move_to_algebraic_notation(
                engine,
                &self.state,
                &chess_move,
                self.state.side_to_move,
            ) {
                self.algebraic_history.push(algebraic);
            } else {
                self.algebraic_history.push("".to_string());
            }
            self.move_history.push(chess_move);
        }

        self.state.play_move(chess_move);

        let (legal_moves, status) = engine.generate_moves(&self.state);
        self.legal_moves = legal_moves.iter().copied().collect();
        self.status = status;

        true
    }

    pub fn is_move_playable(&self, chess_move: ChessMove) -> bool {
        self.legal_moves.contains(&chess_move)
    }

    pub fn find_legal_move(
        &self,
        from: Square,
        to: Square,
        promotion: Option<Piece>,
    ) -> Option<ChessMove> {
        self.legal_moves
            .iter()
            .find(|chess_move| {
                chess_move.get_from() == from.get_value()
                    && chess_move.get_to() == to.get_value()
                    && chess_move.get_type().promotion_piece() == promotion
            })
            .copied()
    }

    pub fn play_move_from_to(
        &mut self,
        engine: &Arc<Engine>,
        from: Square,
        to: Square,
        promotion: Option<Piece>,
    ) -> bool {
        if let Some(chess_move) = self.find_legal_move(from, to, promotion) {
            self.play_move(engine, chess_move)
        } else {
            false
        }
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn winner(&self) -> Option<Color> {
        if self.status == GameStatus::Checkmate {
            Some(self.state.side_to_move.opposite())
        } else {
            None
        }
    }

    pub fn side_to_move(&self) -> Color {
        self.state.side_to_move
    }

    pub fn half_moves(&self) -> u8 {
        self.state.half_moves
    }

    pub fn full_moves(&self) -> u16 {
        self.state.full_moves
    }

    pub fn legal_moves(&self) -> &HashSet<ChessMove> {
        &self.legal_moves
    }

    pub fn legal_moves_algebraic(&self, engine: &Arc<Engine>) -> HashMap<String, ChessMove> {
        self.legal_moves
            .iter()
            .filter_map(|chess_move| {
                Some((
                    parse_move_to_algebraic_notation(
                        engine,
                        &self.state,
                        chess_move,
                        self.state.side_to_move,
                    )?,
                    *chess_move,
                ))
            })
            .collect()
    }

    pub fn legal_move_squares(&self) -> HashMap<Square, Vec<Square>> {
        self.legal_moves
            .iter()
            .fold(HashMap::new(), |mut acc, chess_move| {
                let from = Square::new(chess_move.get_from());
                let to = Square::new(chess_move.get_to());
                acc.entry(from).or_default().push(to);
                acc
            })
    }

    pub fn board(&self) -> ChessBoard {
        self.state.board
    }

    pub fn move_history(&self) -> &Vec<ChessMove> {
        &self.move_history
    }

    pub fn latest_move(&self) -> Option<ChessMove> {
        self.move_history.last().copied()
    }

    pub fn latest_move_algebraic(&self) -> Option<String> {
        Some(self.algebraic_history.last()?.to_string())
    }

    pub fn algebraic_history(&self) -> &Vec<String> {
        &self.algebraic_history
    }

    pub fn get_result_pgn(&self) -> Option<&str> {
        if let Some(winner) = self.winner() {
            match winner {
                Color::White => Some("1-0"),
                Color::Black => Some("0-1"),
            }
        } else if self.status.is_draw() {
            Some("½–½")
        } else {
            None
        }
    }

    pub fn get_pgn(&self) -> String {
        let mut pgn = self
            .pgn_metadata
            .format(self.get_result_pgn(), self.origin_fen.as_deref());

        if !pgn.is_empty() && !self.algebraic_history.is_empty() {
            pgn.push('\n');
        }

        for (i, algebraic_move) in self.algebraic_history.iter().enumerate() {
            let new_round = i % 2 == 0;
            let round = (i / 2) + 1;
            if new_round {
                pgn.push_str(&format!("{}.", round));
            }
            pgn.push_str(&format!("{} ", algebraic_move));
        }

        if let Some(result) = self.get_result_pgn() {
            pgn.push_str(result);
        }

        pgn
    }

    pub fn get_fen_string(&self) -> String {
        self.state.get_fen_string()
    }

    pub fn from_fen_string(engine: &Arc<Engine>, fen_string: &str) -> Result<Self, Box<dyn Error>> {
        let state = GameState::from_fen_string(fen_string)?;
        let (legal_moves, status) = engine.generate_moves(&state);
        Ok(Self {
            state,
            status,
            legal_moves: legal_moves.iter().copied().collect(),
            move_history: Vec::new(),
            algebraic_history: Vec::new(),
            pgn_metadata: PGNMetadata::default(),
            origin_fen: Some(fen_string.to_string()),
        })
    }

    pub fn set_pgn_meta_data(&mut self, pgn_meta_data: PGNMetadata) {
        self.pgn_metadata = pgn_meta_data;
    }

    pub fn archive(&self) -> ArchivedGame {
        ArchivedGame {
            pgn: self.pgn_metadata.clone(),
            origin_fen: self.origin_fen.clone(),
            played_moves: self.move_history.clone(),
        }
    }

    pub fn get_piece_color_at(&self, square: Square) -> Option<(Piece, Color)> {
        self.board().get_piece_at(square.get_value())
    }

    pub fn get_threats(&self, engine: &Arc<Engine>, square: Square) -> Vec<Square> {
        let Some((_, color)) = self.get_piece_color_at(square) else {
            return vec![];
        };

        let threat_board =
            engine.get_square_threats(self.board(), square.get_value(), color.opposite());
        threat_board.iter_set_bits().map(Square::new).collect()
    }

    pub fn get_check_threats(&self, engine: &Arc<Engine>) -> Vec<Square> {
        let king_bb = self.board().get_piece_bb(Piece::King, self.side_to_move());
        let Some(king_index) = king_bb.get_lowest_set_bit() else {
            return vec![];
        };
        self.get_threats(engine, Square::new(king_index))
    }

    pub fn is_check(&self, engine: &Arc<Engine>) -> bool {
        engine.is_in_check(self.board(), self.side_to_move())
    }
}

#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// The minimal information needed to recreate a game.
pub struct ArchivedGame {
    pub pgn: PGNMetadata,
    pub origin_fen: Option<String>,
    pub played_moves: Vec<ChessMove>,
}

impl ArchivedGame {
    pub fn move_count(&self) -> u16 {
        self.played_moves.len() as u16
    }

    pub fn restore(&self, engine: &Arc<Engine>) -> Game {
        self.replay_till(engine, self.move_count())
    }

    pub fn replay_till(&self, engine: &Arc<Engine>, half_moves: u16) -> Game {
        let mut game = if let Some(fen) = &self.origin_fen {
            Game::from_fen_string(engine, fen)
                .unwrap_or_else(|_| Game::new(engine, self.pgn.clone()))
        } else {
            Game::new(engine, self.pgn.clone())
        };

        for chess_move in self.played_moves.iter().take(half_moves as usize) {
            game.play_move(engine, *chess_move);
        }

        game
    }

    pub fn replay_iter(&self, engine: &Arc<Engine>) -> impl Iterator<Item = Game> + '_ {
        let engine = engine.clone();
        (0..=self.move_count()).map(move |move_count| self.replay_till(&engine, move_count))
    }
}
