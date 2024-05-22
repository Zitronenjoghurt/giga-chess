use crate::{bitboard::BitBoard, color::Color, piece::Piece};

// A chessboard representation using 8 bitboards, 1 for every piece and 2 for the colors
// White is always at the bottom of the board
// Indexing starts in the bottom left of the board, to the right
pub struct ChessBoard8 {
    pub colors: [BitBoard; 2],
    pub pieces: [BitBoard; 6],
}

impl Default for ChessBoard8 {
    fn default() -> Self {
        Self {
            colors: [
                BitBoard(0b0000000000000000000000000000000000000000000000001111111111111111),
                BitBoard(0b1111111111111111000000000000000000000000000000000000000000000000),
            ],
            pieces: [
                BitBoard(0b0000000011111111000000000000000000000000000000001111111100000000), // Pawn
                BitBoard(0b0100001000000000000000000000000000000000000000000000000001000010), // Knight
                BitBoard(0b0010010000000000000000000000000000000000000000000000000000100100), // Bishop
                BitBoard(0b1000000100000000000000000000000000000000000000000000000010000001), // Rook
                BitBoard(0b0000100000000000000000000000000000000000000000000000000000001000), // Queen
                BitBoard(0b0001000000000000000000000000000000000000000000000000000000010000), // King
            ],
        }
    }
}

impl ChessBoard8 {
    pub fn piece_at_index(&self, index: u8) -> Piece {
        for (i, mask) in self.pieces.iter().enumerate() {
            if mask.get_bit(index) {
                return Piece::from(i);
            }
        }
        Piece::NONE
    }

    pub fn color_at_index(&self, index: u8) -> Color {
        for (i, mask) in self.colors.iter().enumerate() {
            if mask.get_bit(index) {
                return Color::from(i);
            }
        }
        Color::NONE
    }

    pub fn piece_and_color_at_index(&self, index: u8) -> (Piece, Color) {
        (self.piece_at_index(index), self.color_at_index(index))
    }
}

#[cfg(test)]
mod tests {
    use super::ChessBoard8;
    use crate::{color::Color, piece::Piece};

    const EXPECTED_PIECES: [Piece; 65] = [
        Piece::ROOK,
        Piece::KNIGHT,
        Piece::BISHOP,
        Piece::QUEEN,
        Piece::KING,
        Piece::BISHOP,
        Piece::KNIGHT,
        Piece::ROOK,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::NONE,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::PAWN,
        Piece::ROOK,
        Piece::KNIGHT,
        Piece::BISHOP,
        Piece::QUEEN,
        Piece::KING,
        Piece::BISHOP,
        Piece::KNIGHT,
        Piece::ROOK,
        Piece::NONE,
    ];

    const EXPECTED_COLORS: [Color; 65] = [
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::WHITE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::NONE,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::BLACK,
        Color::NONE,
    ];

    #[test]
    fn test_piece_at_index() {
        let cb = ChessBoard8::default();
        for i in 0..65 {
            assert_eq!(cb.piece_at_index(i), EXPECTED_PIECES[i as usize]);
        }
    }

    #[test]
    fn test_color_at_index() {
        let cb = ChessBoard8::default();
        for i in 0..65 {
            assert_eq!(cb.color_at_index(i), EXPECTED_COLORS[i as usize]);
        }
    }

    #[test]
    fn test_piece_and_color_at_index() {
        let cb = ChessBoard8::default();
        for i in 0..65 {
            assert_eq!(
                cb.piece_and_color_at_index(i),
                (EXPECTED_PIECES[i as usize], EXPECTED_COLORS[i as usize])
            );
        }
    }
}
