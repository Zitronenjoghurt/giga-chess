use crate::game::castling_rights::CastlingRights;
use crate::game::chess_board::ChessBoard;
use crate::game::chess_move::{ChessMove, ChessMoveType};
use crate::game::color::Color;
use crate::game::piece;
use crate::game::square::{A1, A8, E1, E8, H1, H8};

#[derive(Debug, Clone)]
pub struct GameState {
    pub board: ChessBoard,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    /// En passant target square
    /// The square behind a pawn that just made a double move
    pub en_passant_square: Option<u8>,
    /// Half-move clock for 50-move rule
    /// Counts half-moves since last pawn move or capture
    pub half_moves: u8,
    /// Full-move counter, incremented after black's move
    pub full_moves: u16,
    // ToDo: Threefold repetition
}

impl GameState {
    pub fn new(start_color: Color) -> Self {
        Self {
            board: ChessBoard::default(),
            side_to_move: start_color,
            castling_rights: CastlingRights::default(),
            en_passant_square: None,
            half_moves: 0,
            full_moves: 0,
        }
    }

    pub fn play_move(&mut self, chess_move: ChessMove) {
        let move_from = chess_move.get_from();
        let move_to = chess_move.get_to();
        let move_type = chess_move.get_type();
        let Some(moved_piece) = self
            .board
            .get_piece_at_with_color(move_from, self.side_to_move)
        else {
            return;
        };

        self.board = self
            .board
            .play_move(chess_move, self.side_to_move, self.en_passant_square);

        if moved_piece == piece::Piece::Pawn || move_type.is_capture() {
            self.half_moves = 0;
        } else {
            self.half_moves = self.half_moves.wrapping_add(1);
        }

        if self.side_to_move == Color::Black {
            self.full_moves = self.full_moves.wrapping_add(1);
        }

        if move_type == ChessMoveType::DoublePawnPush {
            self.en_passant_square = match self.side_to_move {
                Color::White => Some(move_from - 8),
                Color::Black => Some(move_from + 8),
            }
        } else {
            self.en_passant_square = None;
        }

        if self.castling_rights.white_king_side
            && (move_from == E1 || move_from == H1 || move_to == H1)
        {
            self.castling_rights.white_king_side = false;
        }
        if self.castling_rights.white_queen_side
            && (move_from == E1 || move_from == A1 || move_to == A1)
        {
            self.castling_rights.white_queen_side = false;
        }
        if self.castling_rights.black_king_side
            && (move_from == E8 || move_from == H8 || move_to == H8)
        {
            self.castling_rights.black_king_side = false;
        }
        if self.castling_rights.black_queen_side
            && (move_from == E8 || move_from == A8 || move_to == A8)
        {
            self.castling_rights.black_queen_side = false;
        }

        self.side_to_move = self.side_to_move.opposite();
    }
}
