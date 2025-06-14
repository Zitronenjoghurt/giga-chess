use crate::engine::Engine;
use crate::game::chess_board::ChessBoard;
use crate::game::chess_move::ChessMove;
use crate::game::color::Color;
use crate::game::state::GameState;
use crate::game::status::GameStatus;
use std::collections::HashSet;
use std::sync::Arc;

pub mod bit_board;
pub mod castling_rights;
pub mod chess_board;
pub mod chess_move;
pub mod color;
pub mod piece;
pub mod square;
pub mod state;
pub mod status;

pub struct Game {
    state: GameState,
    status: GameStatus,
    legal_moves: HashSet<ChessMove>,
    move_history: Vec<ChessMove>,
}

impl Game {
    pub fn new(engine: &Arc<Engine>, starting_color: Color) -> Self {
        let state = GameState::new(starting_color);
        let (legal_moves, status) = engine.generate_moves(&state);
        Self {
            state,
            status,
            legal_moves: legal_moves.iter().copied().collect(),
            move_history: Vec::new(),
        }
    }

    pub fn play_move(&mut self, engine: &Arc<Engine>, chess_move: ChessMove) -> bool {
        if !self.legal_moves.contains(&chess_move) {
            return false;
        }

        self.state.play_move(chess_move);

        let (legal_moves, status) = engine.generate_moves(&self.state);
        self.legal_moves = legal_moves.iter().copied().collect();
        self.status = status;
        self.move_history.push(chess_move);

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

    pub fn legal_moves(&self) -> &HashSet<ChessMove> {
        &self.legal_moves
    }

    pub fn board(&self) -> ChessBoard {
        self.state.board
    }
}
