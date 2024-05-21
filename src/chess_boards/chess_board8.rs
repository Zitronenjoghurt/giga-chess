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

    #[test]
    fn test_piece_at_index() {
        let cb = ChessBoard8::default();
        assert_eq!(cb.piece_at_index(0), Piece::ROOK);
        assert_eq!(cb.piece_at_index(1), Piece::KNIGHT);
        assert_eq!(cb.piece_at_index(2), Piece::BISHOP);
        assert_eq!(cb.piece_at_index(3), Piece::QUEEN);
        assert_eq!(cb.piece_at_index(4), Piece::KING);
        assert_eq!(cb.piece_at_index(5), Piece::BISHOP);
        assert_eq!(cb.piece_at_index(6), Piece::KNIGHT);
        assert_eq!(cb.piece_at_index(7), Piece::ROOK);
        assert_eq!(cb.piece_at_index(8), Piece::PAWN);
        assert_eq!(cb.piece_at_index(9), Piece::PAWN);
        assert_eq!(cb.piece_at_index(10), Piece::PAWN);
        assert_eq!(cb.piece_at_index(11), Piece::PAWN);
        assert_eq!(cb.piece_at_index(12), Piece::PAWN);
        assert_eq!(cb.piece_at_index(13), Piece::PAWN);
        assert_eq!(cb.piece_at_index(14), Piece::PAWN);
        assert_eq!(cb.piece_at_index(15), Piece::PAWN);
        assert_eq!(cb.piece_at_index(16), Piece::NONE);
        assert_eq!(cb.piece_at_index(17), Piece::NONE);
        assert_eq!(cb.piece_at_index(18), Piece::NONE);
        assert_eq!(cb.piece_at_index(19), Piece::NONE);
        assert_eq!(cb.piece_at_index(20), Piece::NONE);
        assert_eq!(cb.piece_at_index(21), Piece::NONE);
        assert_eq!(cb.piece_at_index(22), Piece::NONE);
        assert_eq!(cb.piece_at_index(23), Piece::NONE);
        assert_eq!(cb.piece_at_index(24), Piece::NONE);
        assert_eq!(cb.piece_at_index(25), Piece::NONE);
        assert_eq!(cb.piece_at_index(26), Piece::NONE);
        assert_eq!(cb.piece_at_index(27), Piece::NONE);
        assert_eq!(cb.piece_at_index(28), Piece::NONE);
        assert_eq!(cb.piece_at_index(29), Piece::NONE);
        assert_eq!(cb.piece_at_index(30), Piece::NONE);
        assert_eq!(cb.piece_at_index(31), Piece::NONE);
        assert_eq!(cb.piece_at_index(32), Piece::NONE);
        assert_eq!(cb.piece_at_index(33), Piece::NONE);
        assert_eq!(cb.piece_at_index(34), Piece::NONE);
        assert_eq!(cb.piece_at_index(35), Piece::NONE);
        assert_eq!(cb.piece_at_index(36), Piece::NONE);
        assert_eq!(cb.piece_at_index(37), Piece::NONE);
        assert_eq!(cb.piece_at_index(38), Piece::NONE);
        assert_eq!(cb.piece_at_index(39), Piece::NONE);
        assert_eq!(cb.piece_at_index(40), Piece::NONE);
        assert_eq!(cb.piece_at_index(41), Piece::NONE);
        assert_eq!(cb.piece_at_index(42), Piece::NONE);
        assert_eq!(cb.piece_at_index(43), Piece::NONE);
        assert_eq!(cb.piece_at_index(44), Piece::NONE);
        assert_eq!(cb.piece_at_index(45), Piece::NONE);
        assert_eq!(cb.piece_at_index(46), Piece::NONE);
        assert_eq!(cb.piece_at_index(47), Piece::NONE);
        assert_eq!(cb.piece_at_index(48), Piece::PAWN);
        assert_eq!(cb.piece_at_index(49), Piece::PAWN);
        assert_eq!(cb.piece_at_index(50), Piece::PAWN);
        assert_eq!(cb.piece_at_index(51), Piece::PAWN);
        assert_eq!(cb.piece_at_index(52), Piece::PAWN);
        assert_eq!(cb.piece_at_index(53), Piece::PAWN);
        assert_eq!(cb.piece_at_index(54), Piece::PAWN);
        assert_eq!(cb.piece_at_index(55), Piece::PAWN);
        assert_eq!(cb.piece_at_index(56), Piece::ROOK);
        assert_eq!(cb.piece_at_index(57), Piece::KNIGHT);
        assert_eq!(cb.piece_at_index(58), Piece::BISHOP);
        assert_eq!(cb.piece_at_index(59), Piece::QUEEN);
        assert_eq!(cb.piece_at_index(60), Piece::KING);
        assert_eq!(cb.piece_at_index(61), Piece::BISHOP);
        assert_eq!(cb.piece_at_index(62), Piece::KNIGHT);
        assert_eq!(cb.piece_at_index(63), Piece::ROOK);
        assert_eq!(cb.piece_at_index(64), Piece::NONE);
    }

    #[test]
    fn test_color_at_index() {
        let cb = ChessBoard8::default();
        assert_eq!(cb.color_at_index(0), Color::WHITE);
        assert_eq!(cb.color_at_index(1), Color::WHITE);
        assert_eq!(cb.color_at_index(2), Color::WHITE);
        assert_eq!(cb.color_at_index(3), Color::WHITE);
        assert_eq!(cb.color_at_index(4), Color::WHITE);
        assert_eq!(cb.color_at_index(5), Color::WHITE);
        assert_eq!(cb.color_at_index(6), Color::WHITE);
        assert_eq!(cb.color_at_index(7), Color::WHITE);
        assert_eq!(cb.color_at_index(8), Color::WHITE);
        assert_eq!(cb.color_at_index(9), Color::WHITE);
        assert_eq!(cb.color_at_index(10), Color::WHITE);
        assert_eq!(cb.color_at_index(11), Color::WHITE);
        assert_eq!(cb.color_at_index(12), Color::WHITE);
        assert_eq!(cb.color_at_index(13), Color::WHITE);
        assert_eq!(cb.color_at_index(14), Color::WHITE);
        assert_eq!(cb.color_at_index(15), Color::WHITE);
        assert_eq!(cb.color_at_index(16), Color::NONE);
        assert_eq!(cb.color_at_index(17), Color::NONE);
        assert_eq!(cb.color_at_index(18), Color::NONE);
        assert_eq!(cb.color_at_index(19), Color::NONE);
        assert_eq!(cb.color_at_index(20), Color::NONE);
        assert_eq!(cb.color_at_index(21), Color::NONE);
        assert_eq!(cb.color_at_index(22), Color::NONE);
        assert_eq!(cb.color_at_index(23), Color::NONE);
        assert_eq!(cb.color_at_index(24), Color::NONE);
        assert_eq!(cb.color_at_index(25), Color::NONE);
        assert_eq!(cb.color_at_index(26), Color::NONE);
        assert_eq!(cb.color_at_index(27), Color::NONE);
        assert_eq!(cb.color_at_index(28), Color::NONE);
        assert_eq!(cb.color_at_index(29), Color::NONE);
        assert_eq!(cb.color_at_index(30), Color::NONE);
        assert_eq!(cb.color_at_index(31), Color::NONE);
        assert_eq!(cb.color_at_index(32), Color::NONE);
        assert_eq!(cb.color_at_index(33), Color::NONE);
        assert_eq!(cb.color_at_index(34), Color::NONE);
        assert_eq!(cb.color_at_index(35), Color::NONE);
        assert_eq!(cb.color_at_index(36), Color::NONE);
        assert_eq!(cb.color_at_index(37), Color::NONE);
        assert_eq!(cb.color_at_index(38), Color::NONE);
        assert_eq!(cb.color_at_index(39), Color::NONE);
        assert_eq!(cb.color_at_index(40), Color::NONE);
        assert_eq!(cb.color_at_index(41), Color::NONE);
        assert_eq!(cb.color_at_index(42), Color::NONE);
        assert_eq!(cb.color_at_index(43), Color::NONE);
        assert_eq!(cb.color_at_index(44), Color::NONE);
        assert_eq!(cb.color_at_index(45), Color::NONE);
        assert_eq!(cb.color_at_index(46), Color::NONE);
        assert_eq!(cb.color_at_index(47), Color::NONE);
        assert_eq!(cb.color_at_index(48), Color::BLACK);
        assert_eq!(cb.color_at_index(49), Color::BLACK);
        assert_eq!(cb.color_at_index(50), Color::BLACK);
        assert_eq!(cb.color_at_index(51), Color::BLACK);
        assert_eq!(cb.color_at_index(52), Color::BLACK);
        assert_eq!(cb.color_at_index(53), Color::BLACK);
        assert_eq!(cb.color_at_index(54), Color::BLACK);
        assert_eq!(cb.color_at_index(55), Color::BLACK);
        assert_eq!(cb.color_at_index(56), Color::BLACK);
        assert_eq!(cb.color_at_index(57), Color::BLACK);
        assert_eq!(cb.color_at_index(58), Color::BLACK);
        assert_eq!(cb.color_at_index(59), Color::BLACK);
        assert_eq!(cb.color_at_index(60), Color::BLACK);
        assert_eq!(cb.color_at_index(61), Color::BLACK);
        assert_eq!(cb.color_at_index(62), Color::BLACK);
        assert_eq!(cb.color_at_index(63), Color::BLACK);
        assert_eq!(cb.color_at_index(64), Color::NONE);
    }

    #[test]
    fn test_piece_and_color_at_index() {
        let cb = ChessBoard8::default();
        assert_eq!(cb.piece_and_color_at_index(0), (Piece::ROOK, Color::WHITE));
        assert_eq!(
            cb.piece_and_color_at_index(1),
            (Piece::KNIGHT, Color::WHITE)
        );
        assert_eq!(
            cb.piece_and_color_at_index(2),
            (Piece::BISHOP, Color::WHITE)
        );
        assert_eq!(cb.piece_and_color_at_index(3), (Piece::QUEEN, Color::WHITE));
        assert_eq!(cb.piece_and_color_at_index(4), (Piece::KING, Color::WHITE));
        assert_eq!(
            cb.piece_and_color_at_index(5),
            (Piece::BISHOP, Color::WHITE)
        );
        assert_eq!(
            cb.piece_and_color_at_index(6),
            (Piece::KNIGHT, Color::WHITE)
        );
        assert_eq!(cb.piece_and_color_at_index(7), (Piece::ROOK, Color::WHITE));
        assert_eq!(cb.piece_and_color_at_index(8), (Piece::PAWN, Color::WHITE));
        assert_eq!(cb.piece_and_color_at_index(9), (Piece::PAWN, Color::WHITE));
        assert_eq!(cb.piece_and_color_at_index(10), (Piece::PAWN, Color::WHITE));
        assert_eq!(cb.piece_and_color_at_index(11), (Piece::PAWN, Color::WHITE));
        assert_eq!(cb.piece_and_color_at_index(12), (Piece::PAWN, Color::WHITE));
        assert_eq!(cb.piece_and_color_at_index(13), (Piece::PAWN, Color::WHITE));
        assert_eq!(cb.piece_and_color_at_index(14), (Piece::PAWN, Color::WHITE));
        assert_eq!(cb.piece_and_color_at_index(15), (Piece::PAWN, Color::WHITE));
        assert_eq!(cb.piece_and_color_at_index(16), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(17), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(18), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(19), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(20), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(21), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(22), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(23), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(24), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(25), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(26), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(27), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(28), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(29), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(30), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(31), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(32), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(33), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(34), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(35), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(36), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(37), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(38), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(39), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(40), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(41), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(42), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(43), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(44), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(45), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(46), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(47), (Piece::NONE, Color::NONE));
        assert_eq!(cb.piece_and_color_at_index(48), (Piece::PAWN, Color::BLACK));
        assert_eq!(cb.piece_and_color_at_index(49), (Piece::PAWN, Color::BLACK));
        assert_eq!(cb.piece_and_color_at_index(50), (Piece::PAWN, Color::BLACK));
        assert_eq!(cb.piece_and_color_at_index(51), (Piece::PAWN, Color::BLACK));
        assert_eq!(cb.piece_and_color_at_index(52), (Piece::PAWN, Color::BLACK));
        assert_eq!(cb.piece_and_color_at_index(53), (Piece::PAWN, Color::BLACK));
        assert_eq!(cb.piece_and_color_at_index(54), (Piece::PAWN, Color::BLACK));
        assert_eq!(cb.piece_and_color_at_index(55), (Piece::PAWN, Color::BLACK));
        assert_eq!(cb.piece_and_color_at_index(56), (Piece::ROOK, Color::BLACK));
        assert_eq!(
            cb.piece_and_color_at_index(57),
            (Piece::KNIGHT, Color::BLACK)
        );
        assert_eq!(
            cb.piece_and_color_at_index(58),
            (Piece::BISHOP, Color::BLACK)
        );
        assert_eq!(
            cb.piece_and_color_at_index(59),
            (Piece::QUEEN, Color::BLACK)
        );
        assert_eq!(cb.piece_and_color_at_index(60), (Piece::KING, Color::BLACK));
        assert_eq!(
            cb.piece_and_color_at_index(61),
            (Piece::BISHOP, Color::BLACK)
        );
        assert_eq!(
            cb.piece_and_color_at_index(62),
            (Piece::KNIGHT, Color::BLACK)
        );
        assert_eq!(cb.piece_and_color_at_index(63), (Piece::ROOK, Color::BLACK));
        assert_eq!(cb.piece_and_color_at_index(64), (Piece::NONE, Color::NONE));
    }
}
