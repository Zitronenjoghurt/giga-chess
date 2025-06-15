use crate::game::bit_board::BitBoard;
use crate::game::chess_move::{ChessMove, ChessMoveType};
use crate::game::color::{Color, COLORS};
use crate::game::piece::{Piece, PIECES};
use crate::game::square::*;
use std::error::Error;
use std::fmt::{Display, Formatter};

const DEFAULT_BOARD: ChessBoard = ChessBoard([
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000),
    BitBoard::new(0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
]);

/// A chess board containing 12 bit boards for each piece and color.
/// Square indexing starts with 0 at A1, 1 at B1, ... and ends with 63 at H8.
/// The bit boards are indexed as follows:
/// 0: White Pawns
/// 1: White Knights
/// 2: White Bishops
/// 3: White Rooks
/// 4: White Queens
/// 5: White Kings
/// 6: Black Pawns
/// 7: Black Knights
/// 8: Black Bishops
/// 9: Black Rooks
/// 10: Black Queens
/// 11: Black Kings
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct ChessBoard([BitBoard; 12]);

impl ChessBoard {
    #[inline(always)]
    pub const fn new(bit_boards: [BitBoard; 12]) -> Self {
        Self(bit_boards)
    }

    #[inline(always)]
    pub const fn empty() -> Self {
        Self([BitBoard::empty(); 12])
    }

    #[inline(always)]
    pub const fn default() -> Self {
        DEFAULT_BOARD
    }

    #[inline(always)]
    pub fn get_piece_bb(&self, piece: Piece, color: Color) -> BitBoard {
        self.0[piece as usize + (color as usize * 6)]
    }

    #[inline(always)]
    pub fn get_piece_bb_mut(&mut self, piece: Piece, color: Color) -> &mut BitBoard {
        &mut self.0[piece as usize + (color as usize * 6)]
    }

    #[inline(always)]
    pub fn get_color_bb(&self, color: Color) -> BitBoard {
        let base = color as usize * 6;
        self.0[base]
            | self.0[base + 1]
            | self.0[base + 2]
            | self.0[base + 3]
            | self.0[base + 4]
            | self.0[base + 5]
    }

    #[inline(always)]
    pub fn get_occupied_bb(&self) -> BitBoard {
        self.get_color_bb(Color::White) | self.get_color_bb(Color::Black)
    }

    #[inline(always)]
    pub fn set_piece(&mut self, piece: Piece, color: Color, square: u8) {
        self.get_piece_bb_mut(piece, color).set_bit(square);
    }

    #[inline(always)]
    pub fn clear_piece(&mut self, piece: Piece, color: Color, square: u8) {
        self.get_piece_bb_mut(piece, color).clear_bit(square);
    }

    #[inline(always)]
    pub fn move_piece(&mut self, piece: Piece, color: Color, from: u8, to: u8) {
        let bb = self.get_piece_bb_mut(piece, color);
        bb.clear_bit(from);
        bb.set_bit(to);
    }

    #[inline(always)]
    pub fn get_piece_at(&self, square: u8) -> Option<(Piece, Color)> {
        for color in COLORS {
            if let Some(piece) = self.get_piece_at_with_color(square, color) {
                return Some((piece, color));
            }
        }
        None
    }

    #[inline(always)]
    pub fn get_piece_at_with_color(&self, square: u8, color: Color) -> Option<Piece> {
        PIECES
            .into_iter()
            .find(|&piece| self.get_piece_bb(piece, color).get_bit(square))
    }

    fn move_color(&mut self, from: u8, to: u8, color: Color) {
        let Some(piece) = self.get_piece_at_with_color(from, color) else {
            return;
        };
        self.move_piece(piece, color, from, to);
    }

    fn double_pawn_push(&mut self, from: u8, color: Color) {
        let to = match color {
            Color::White => from + 16,
            Color::Black => from - 16,
        };
        self.move_piece(Piece::Pawn, color, from, to);
    }

    fn capture(&mut self, target_square: u8, target_color: Color) {
        let Some(captured_piece) = self.get_piece_at_with_color(target_square, target_color) else {
            return;
        };
        self.clear_piece(captured_piece, target_color, target_square);
    }

    fn move_color_capture(&mut self, from: u8, to: u8, color: Color) {
        self.move_color(from, to, color);
        self.capture(to, color.opposite());
    }

    fn castle_queenside(&mut self, color: Color) {
        match color {
            Color::White => {
                self.move_piece(Piece::King, color, 4, 2);
                self.move_piece(Piece::Rook, color, 0, 3);
            }
            Color::Black => {
                self.move_piece(Piece::King, color, 60, 58);
                self.move_piece(Piece::Rook, color, 56, 59);
            }
        }
    }

    fn castle_kingside(&mut self, color: Color) {
        match color {
            Color::White => {
                self.move_piece(Piece::King, color, 4, 6);
                self.move_piece(Piece::Rook, color, 7, 5);
            }
            Color::Black => {
                self.move_piece(Piece::King, color, 60, 62);
                self.move_piece(Piece::Rook, color, 63, 61);
            }
        }
    }

    /// The en passant target square is the square behind the pawn that moved two squares.
    fn en_passant_capture(&mut self, from: u8, to: u8, color: Color) {
        self.move_piece(Piece::Pawn, color, from, to);
        let captured_pawn_square = match color {
            Color::White => to - 8,
            Color::Black => to + 8,
        };
        self.clear_piece(Piece::Pawn, color.opposite(), captured_pawn_square);
    }

    fn move_promote(&mut self, from: u8, to: u8, color: Color, promotion_piece: Piece) {
        let Some(source_piece) = self.get_piece_at_with_color(from, color) else {
            return;
        };
        self.clear_piece(source_piece, color, from);
        self.set_piece(promotion_piece, color, to);
    }

    fn move_promote_capture(&mut self, from: u8, to: u8, color: Color, promotion_piece: Piece) {
        self.move_promote(from, to, color, promotion_piece);
        self.capture(to, color.opposite());
    }

    /// The chess board is dumb; it assumes that the moves it receives are valid.
    pub fn play_move(&self, chess_move: ChessMove, color: Color) -> Self {
        let mut new_board = *self;
        let from = chess_move.get_from();
        let to = chess_move.get_to();
        let move_type = chess_move.get_type();

        match move_type {
            ChessMoveType::Quiet => {
                new_board.move_color(from, to, color);
            }
            ChessMoveType::DoublePawnPush => {
                new_board.double_pawn_push(from, color);
            }
            ChessMoveType::QueenCastle => {
                new_board.castle_queenside(color);
            }
            ChessMoveType::KingCastle => {
                new_board.castle_kingside(color);
            }
            ChessMoveType::Capture => {
                new_board.move_color_capture(from, to, color);
            }
            ChessMoveType::EnPassant => {
                new_board.en_passant_capture(from, to, color);
            }
            _ => {
                if let Some(promotion_piece) = move_type.promotion_piece() {
                    if move_type.is_capture() {
                        new_board.move_promote_capture(from, to, color, promotion_piece);
                    } else {
                        new_board.move_promote(from, to, color, promotion_piece);
                    }
                }
            }
        }

        new_board
    }

    pub fn get_fen_string(&self) -> String {
        let mut fen = String::new();

        for rank in (1..=8).rev() {
            let mut empty_count = 0;
            let mut rank_string = String::new();

            for file in 1..=8 {
                let square = Square::from_file_rank(file, rank);
                if let Some((piece, color)) = self.get_piece_at(square.get_value()) {
                    if empty_count > 0 {
                        rank_string.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    rank_string.push(piece.get_fen_char(color));
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                rank_string.push_str(&empty_count.to_string());
            }

            if rank > 1 {
                rank_string.push('/');
            }

            fen.push_str(&rank_string);
        }

        fen
    }

    pub fn from_fen_string(fen: &str) -> Result<Self, Box<dyn Error>> {
        let mut board = Self::empty();

        let mut rank = 8;
        let mut file = 1;
        for current_char in fen.chars() {
            if current_char == '/' {
                rank -= 1;
                file = 1;
                continue;
            }

            if file > 8 {
                return Err(format!("File exceeds H in rank {rank}").into());
            }

            if let Ok(piece) = Piece::try_from(current_char) {
                let color = if current_char.is_ascii_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                let square = Square::from_file_rank(file, rank);
                board.set_piece(piece, color, square.get_value());
                file += 1;
            } else if let Some(count) = current_char.to_digit(10) {
                if count > 8 || file + count as u8 > 9 {
                    return Err(format!("Count exceeds 8 in rank {rank}").into());
                }
                file += count as u8;
            } else {
                return Err(format!("Invalid character '{current_char}' in rank {rank}").into());
            }
        }

        Ok(board)
    }
}

impl Display for ChessBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for rank in (1..=8).rev() {
            write!(f, "{} ", rank)?;
            for file in 1..=8 {
                let square = Square::from_file_rank(file, rank);
                if let Some((piece, color)) = self.get_piece_at(square.get_value()) {
                    write!(f, "{} ", piece.get_icon(color))?;
                } else if square.is_white() {
                    write!(f, "□ ")?;
                } else {
                    write!(f, "■ ")?;
                };
            }
            writeln!(f)?;
        }
        writeln!(f, "  A B C D E F G H")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::game::bit_board::BitBoard;
    use crate::game::chess_board::ChessBoard;
    use crate::game::chess_move::{ChessMove, ChessMoveType};
    use crate::game::color::Color;
    use crate::game::piece::Piece;
    use crate::game::square::{A8, B7, B8, C2, C4, C5, C8, D5, D6, D7, D8, E8};

    const BOARDS: [BitBoard; 12] = [
        BitBoard::new(0b000000_000001),
        BitBoard::new(0b000000_000010),
        BitBoard::new(0b000000_000100),
        BitBoard::new(0b000000_001000),
        BitBoard::new(0b000000_010000),
        BitBoard::new(0b000000_100000),
        BitBoard::new(0b000001_000000),
        BitBoard::new(0b000010_000000),
        BitBoard::new(0b000100_000000),
        BitBoard::new(0b001000_000000),
        BitBoard::new(0b010000_000000),
        BitBoard::new(0b100000_000000),
    ];

    #[test]
    fn test_get_piece_bb() {
        let board = ChessBoard::new(BOARDS);
        assert_eq!(
            board.get_piece_bb(Piece::Pawn, Color::White).get_value(),
            0b000000_000001
        );
        assert_eq!(
            board.get_piece_bb(Piece::Pawn, Color::Black).get_value(),
            0b000001_000000
        );
        assert_eq!(
            board.get_piece_bb(Piece::Knight, Color::White).get_value(),
            0b000000_000010
        );
        assert_eq!(
            board.get_piece_bb(Piece::Knight, Color::Black).get_value(),
            0b000010_000000
        );
        assert_eq!(
            board.get_piece_bb(Piece::Bishop, Color::White).get_value(),
            0b000000_000100
        );
        assert_eq!(
            board.get_piece_bb(Piece::Bishop, Color::Black).get_value(),
            0b000100_000000
        );
        assert_eq!(
            board.get_piece_bb(Piece::Rook, Color::White).get_value(),
            0b000000_001000
        );
        assert_eq!(
            board.get_piece_bb(Piece::Rook, Color::Black).get_value(),
            0b001000_000000
        );
        assert_eq!(
            board.get_piece_bb(Piece::Queen, Color::White).get_value(),
            0b000000_010000
        );
        assert_eq!(
            board.get_piece_bb(Piece::Queen, Color::Black).get_value(),
            0b010000_000000
        );
        assert_eq!(
            board.get_piece_bb(Piece::King, Color::White).get_value(),
            0b000000_100000
        );
        assert_eq!(
            board.get_piece_bb(Piece::King, Color::Black).get_value(),
            0b100000_000000
        );
    }

    #[test]
    fn test_get_color_bb() {
        let board = ChessBoard::new(BOARDS);
        assert_eq!(
            board.get_color_bb(Color::White).get_value(),
            0b000000_111111
        );
        assert_eq!(
            board.get_color_bb(Color::Black).get_value(),
            0b111111_000000
        );
    }

    #[test]
    fn test_get_occupied_bb() {
        let board = ChessBoard::new(BOARDS);
        assert_eq!(board.get_occupied_bb().get_value(), 0b111111_111111);
    }

    #[test]
    fn test_set_piece() {
        let mut board = ChessBoard::empty();
        board.set_piece(Piece::Pawn, Color::White, 0);
        assert_eq!(
            board.get_piece_bb(Piece::Pawn, Color::White).get_value(),
            0b000000_000001
        );
        board.set_piece(Piece::Pawn, Color::Black, 1);
        assert_eq!(
            board.get_piece_bb(Piece::Pawn, Color::Black).get_value(),
            0b000000_000010
        );
    }

    #[test]
    fn test_clear_piece() {
        let mut board = ChessBoard::new(BOARDS);
        board.clear_piece(Piece::Pawn, Color::White, 0);
        board.clear_piece(Piece::Knight, Color::White, 1);
        board.clear_piece(Piece::Bishop, Color::White, 2);
        board.clear_piece(Piece::Rook, Color::White, 3);
        board.clear_piece(Piece::Queen, Color::White, 4);
        board.clear_piece(Piece::King, Color::White, 5);
        board.clear_piece(Piece::Pawn, Color::Black, 6);
        board.clear_piece(Piece::Knight, Color::Black, 7);
        board.clear_piece(Piece::Bishop, Color::Black, 8);
        board.clear_piece(Piece::Rook, Color::Black, 9);
        board.clear_piece(Piece::Queen, Color::Black, 10);
        board.clear_piece(Piece::King, Color::Black, 11);
        assert_eq!(board.get_occupied_bb().get_value(), 0);
    }

    #[test]
    fn test_get_piece_at() {
        let board = ChessBoard::new(BOARDS);
        assert_eq!(board.get_piece_at(0).unwrap(), (Piece::Pawn, Color::White));
        assert_eq!(
            board.get_piece_at(1).unwrap(),
            (Piece::Knight, Color::White)
        );
        assert_eq!(
            board.get_piece_at(2).unwrap(),
            (Piece::Bishop, Color::White)
        );
        assert_eq!(board.get_piece_at(3).unwrap(), (Piece::Rook, Color::White));
        assert_eq!(board.get_piece_at(4).unwrap(), (Piece::Queen, Color::White));
        assert_eq!(board.get_piece_at(5).unwrap(), (Piece::King, Color::White));
        assert_eq!(board.get_piece_at(6).unwrap(), (Piece::Pawn, Color::Black));
        assert_eq!(
            board.get_piece_at(7).unwrap(),
            (Piece::Knight, Color::Black)
        );
        assert_eq!(
            board.get_piece_at(8).unwrap(),
            (Piece::Bishop, Color::Black)
        );
        assert_eq!(board.get_piece_at(9).unwrap(), (Piece::Rook, Color::Black));
        assert_eq!(
            board.get_piece_at(10).unwrap(),
            (Piece::Queen, Color::Black)
        );
        assert_eq!(board.get_piece_at(11).unwrap(), (Piece::King, Color::Black));
        assert_eq!(board.get_piece_at(12), None);
    }

    #[test]
    fn test_play_move() {
        let board = ChessBoard::default();

        let m = ChessMove::new(C2, 0, ChessMoveType::DoublePawnPush);
        let board = board.play_move(m, Color::White);
        assert_eq!(board.get_piece_at(C2), None);
        assert_eq!(board.get_piece_at(C4).unwrap(), (Piece::Pawn, Color::White));

        let m = ChessMove::new(C4, C5, ChessMoveType::Quiet);
        let board = board.play_move(m, Color::White);
        assert_eq!(board.get_piece_at(C4), None);
        assert_eq!(board.get_piece_at(C5).unwrap(), (Piece::Pawn, Color::White));

        let m = ChessMove::new(D7, 0, ChessMoveType::DoublePawnPush);
        let board = board.play_move(m, Color::Black);
        assert_eq!(board.get_piece_at(D7), None);
        assert_eq!(board.get_piece_at(D5).unwrap(), (Piece::Pawn, Color::Black));

        let m = ChessMove::new(C5, D6, ChessMoveType::EnPassant);
        let board = board.play_move(m, Color::White);
        assert_eq!(board.get_piece_at(C5), None);
        assert_eq!(board.get_piece_at(D5), None);
        assert_eq!(board.get_piece_at(D6).unwrap(), (Piece::Pawn, Color::White));

        let m = ChessMove::new(D6, D7, ChessMoveType::Quiet);
        let board = board.play_move(m, Color::White);
        assert_eq!(board.get_piece_at(D6), None);
        assert_eq!(board.get_piece_at(D7).unwrap(), (Piece::Pawn, Color::White));

        let m = ChessMove::new(D7, C8, ChessMoveType::QueenPromotionCapture);
        assert_eq!(
            board.get_piece_at(C8).unwrap(),
            (Piece::Bishop, Color::Black)
        );
        let board = board.play_move(m, Color::White);
        assert_eq!(board.get_piece_at(D7), None);
        assert_eq!(
            board.get_piece_at(C8).unwrap(),
            (Piece::Queen, Color::White)
        );

        let m = ChessMove::new(C8, D8, ChessMoveType::Capture);
        assert_eq!(
            board.get_piece_at(D8).unwrap(),
            (Piece::Queen, Color::Black)
        );
        let board = board.play_move(m, Color::White);
        assert_eq!(board.get_piece_at(C8), None);
        assert_eq!(
            board.get_piece_at(D8).unwrap(),
            (Piece::Queen, Color::White)
        );

        let m = ChessMove::new(D8, B8, ChessMoveType::Capture);
        assert_eq!(
            board.get_piece_at(B8).unwrap(),
            (Piece::Knight, Color::Black)
        );
        let board = board.play_move(m, Color::White);
        assert_eq!(board.get_piece_at(D8), None);
        assert_eq!(
            board.get_piece_at(B8).unwrap(),
            (Piece::Queen, Color::White)
        );

        let m = ChessMove::new(B8, B7, ChessMoveType::Capture);
        assert_eq!(board.get_piece_at(B7).unwrap(), (Piece::Pawn, Color::Black));
        let board = board.play_move(m, Color::White);
        assert_eq!(board.get_piece_at(B8), None);
        assert_eq!(
            board.get_piece_at(B7).unwrap(),
            (Piece::Queen, Color::White)
        );

        let m = ChessMove::new(0, 0, ChessMoveType::QueenCastle);
        let board = board.play_move(m, Color::Black);
        assert_eq!(board.get_piece_at(A8), None);
        assert_eq!(board.get_piece_at(E8), None);
        assert_eq!(board.get_piece_at(C8).unwrap(), (Piece::King, Color::Black));
        assert_eq!(board.get_piece_at(D8).unwrap(), (Piece::Rook, Color::Black));
    }
}
