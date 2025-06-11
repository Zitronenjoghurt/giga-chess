use crate::types::bitboard::BitBoard;
use crate::types::color::{Color, COLORS};
use crate::types::piece::{Piece, PIECES};

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
        self.0[base..base + 6]
            .iter()
            .fold(BitBoard::empty(), |acc, &bb| acc | bb)
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

#[cfg(test)]
mod tests {
    use crate::types::bitboard::BitBoard;
    use crate::types::chess_board::ChessBoard;
    use crate::types::color::Color;
    use crate::types::piece::Piece;

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
