use crate::{bitboard::BitBoard, color::Color, piece::Piece};

// A chessboard representation using 12 bitboards, 1 for every piece and color
// White is always at the bottom of the board
// Indexing starts in the bottom left of the board, to the right
pub struct ChessBoard12(pub [BitBoard; 12]);

impl Default for ChessBoard12 {
    fn default() -> Self {
        Self([
            BitBoard(0b0000000000000000000000000000000000000000000000001111111100000000), // White Pawns
            BitBoard(0b0000000000000000000000000000000000000000000000000000000001000010), // White Knights
            BitBoard(0b0000000000000000000000000000000000000000000000000000000000100100), // White Bishops
            BitBoard(0b0000000000000000000000000000000000000000000000000000000010000001), // White Rooks
            BitBoard(0b0000000000000000000000000000000000000000000000000000000000001000), // White Queen
            BitBoard(0b0000000000000000000000000000000000000000000000000000000000010000), // White King
            BitBoard(0b0000000011111111000000000000000000000000000000000000000000000000), // Black Pawns
            BitBoard(0b0100001000000000000000000000000000000000000000000000000000000000), // Black Knights
            BitBoard(0b0010010000000000000000000000000000000000000000000000000000000000), // Black Bishops
            BitBoard(0b1000000100000000000000000000000000000000000000000000000000000000), // Black Rooks
            BitBoard(0b0000100000000000000000000000000000000000000000000000000000000000), // Black Queen
            BitBoard(0b0001000000000000000000000000000000000000000000000000000000000000), // Black King
        ])
    }
}

impl ChessBoard12 {
    pub fn piece_at_index(&self, index: u8) -> Piece {
        for (i, mask) in self.0.iter().enumerate() {
            if mask.get_bit(index) {
                if i > 5 {
                    return Piece::from(i - 6);
                } else {
                    return Piece::from(i);
                }
            }
        }
        Piece::NONE
    }

    pub fn color_at_index(&self, index: u8) -> Color {
        for (i, mask) in self.0.iter().enumerate() {
            if mask.get_bit(index) {
                if i > 5 {
                    return Color::BLACK;
                } else {
                    return Color::WHITE;
                }
            }
        }
        Color::NONE
    }

    pub fn piece_and_color_at_index(&self, index: u8) -> (Piece, Color) {
        for (i, mask) in self.0.iter().enumerate() {
            if mask.get_bit(index) {
                if i > 5 {
                    return (Piece::from(i - 6), Color::BLACK);
                } else {
                    return (Piece::from(i), Color::WHITE);
                }
            }
        }
        (Piece::NONE, Color::NONE)
    }
}

#[cfg(test)]
mod tests {
    use super::ChessBoard12;
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
        let cb = ChessBoard12::default();
        for i in 0..65 {
            assert_eq!(cb.piece_at_index(i), EXPECTED_PIECES[i as usize]);
        }
    }

    #[test]
    fn test_color_at_index() {
        let cb = ChessBoard12::default();
        for i in 0..65 {
            assert_eq!(cb.color_at_index(i), EXPECTED_COLORS[i as usize]);
        }
    }

    #[test]
    fn test_piece_and_color_at_index() {
        let cb = ChessBoard12::default();
        for i in 0..65 {
            assert_eq!(
                cb.piece_and_color_at_index(i),
                (EXPECTED_PIECES[i as usize], EXPECTED_COLORS[i as usize])
            );
        }
    }
}
