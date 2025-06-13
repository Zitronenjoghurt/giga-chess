use castling_rights::CastlingRights;
use chess_board::ChessBoard;
use color::Color;

pub mod bit_board;
pub mod castling_rights;
pub mod chess_board;
pub mod chess_move;
pub mod color;
pub mod piece;
pub mod square;

#[derive(Debug, Clone)]
pub struct Game {
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
}
