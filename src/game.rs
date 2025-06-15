use crate::engine::Engine;
use crate::game::algebraic_notation::parse_move_to_algebraic_notation;
use crate::game::chess_board::ChessBoard;
use crate::game::chess_move::ChessMove;
use crate::game::color::Color;
use crate::game::pgn_metadata::PGNMetadata;
use crate::game::state::GameState;
use crate::game::status::GameStatus;
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

#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
            pgn.push_str("\n");
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
            pgn.push_str(&format!("{}", result));
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
}
