use crate::engine::bit_board::BitBoard;
use crate::engine::square::Square;
use crate::game::color::{Color, COLORS};
use crate::game::piece::{Piece, PIECES};
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
#[derive(Debug, Copy, Clone)]
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
            for piece in PIECES {
                if self.get_piece_bb(piece, color).get_bit(square) {
                    return Some((piece, color));
                }
            }
        }
        None
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
                } else {
                    if square.is_white() {
                        write!(f, "□ ")?;
                    } else {
                        write!(f, "■ ")?;
                    }
                };
            }
            write!(f, "\n")?;
        }
        write!(f, "  A B C D E F G H\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::bit_board::BitBoard;
    use crate::game::chess_board::ChessBoard;
    use crate::game::color::Color;
    use crate::game::piece::Piece;

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
}
