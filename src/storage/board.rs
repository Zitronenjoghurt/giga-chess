use crate::core::bitboard::BitBoard;
use crate::prelude::{ChessBoard, Color, Piece, Square, DEFAULT_BOARD};
use crate::storage::io::{BitDecode, BitEncode, BitReader, BitWriter};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum StoredBoardTag {
    StartPosition = 0b00,
    Sparse = 0b01,
    Dense = 0b10,
    DenseHuffman = 0b11,
}

impl StoredBoardTag {
    pub fn cost(&self, board: &ChessBoard) -> usize {
        match self {
            Self::StartPosition => 0,
            Self::Sparse => 6 + 10 * board.total_piece_count() as usize,
            Self::Dense => 64 + 4 * board.total_piece_count() as usize,
            Self::DenseHuffman => {
                let occupied = board.occupied_bb().count_set() as usize;
                let pawns = board.count_pawns() as usize;
                let other = occupied - pawns;
                let empty = 64 - occupied;
                empty + pawns * 3 + other * 6
            }
        }
    }

    pub fn most_optimal(board: &ChessBoard) -> Self {
        if *board == DEFAULT_BOARD {
            return Self::StartPosition;
        }
        [Self::Sparse, Self::Dense, Self::DenseHuffman]
            .into_iter()
            .min_by_key(|tag| tag.cost(board))
            .unwrap()
    }

    pub fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            0b00 => Some(Self::StartPosition),
            0b01 => Some(Self::Sparse),
            0b10 => Some(Self::Dense),
            0b11 => Some(Self::DenseHuffman),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StoredBoard(Vec<u8>);

impl StoredBoard {
    pub fn new(board: &ChessBoard) -> std::io::Result<Self> {
        let mut buffer = Vec::new();
        {
            let mut writer = BitWriter::new(&mut buffer);
            board.encode(&mut writer)?;
            writer.flush()?;
        }
        Ok(Self(buffer))
    }

    pub fn restore(self) -> std::io::Result<ChessBoard> {
        let mut reader = BitReader::new(self.0.as_slice());
        ChessBoard::decode(&mut reader)
    }

    pub fn tag(&self) -> Option<StoredBoardTag> {
        StoredBoardTag::from_bits(self.0[0] >> 6)
    }
}

impl BitEncode for ChessBoard {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<()> {
        let occupied = self.occupied_bb();
        let piece_count = occupied.count_set();

        let tag = StoredBoardTag::most_optimal(self);
        w.write_bits(tag as u8, 2)?;

        match tag {
            StoredBoardTag::StartPosition => {}
            StoredBoardTag::Sparse => {
                w.write_bits(piece_count, 6)?;
                for (square, (piece, color)) in self.iter_all_pieces_bottom_top() {
                    w.write(&square)?;
                    w.write(&color)?;
                    w.write(&piece)?;
                }
            }
            StoredBoardTag::Dense => {
                w.write(&occupied)?;
                for (piece, color) in self
                    .iter_all_pieces_bottom_top()
                    .map(|(_, piece_color)| piece_color)
                {
                    w.write(&color)?;
                    w.write(&piece)?;
                }
            }
            StoredBoardTag::DenseHuffman => {
                for (_, opt_piece_color) in self.iter_bottom_top(Color::White) {
                    if let Some((piece, color)) = opt_piece_color {
                        if piece == Piece::Pawn {
                            w.write_bits(0b10u8, 2)?;
                            w.write(&color)?;
                        } else {
                            let piece_bits: u8 = match piece {
                                Piece::Knight => 0b11_000,
                                Piece::Bishop => 0b11_001,
                                Piece::Rook => 0b11_010,
                                Piece::Queen => 0b11_011,
                                Piece::King => 0b11_100,
                                _ => unreachable!(),
                            };
                            w.write_bits(piece_bits, 5)?;
                            w.write(&color)?;
                        }
                    } else {
                        w.write(&false)?;
                    }
                }
            }
        }

        Ok(())
    }
}

impl BitDecode for ChessBoard {
    fn decode<R: Read>(r: &mut BitReader<R>) -> std::io::Result<Self> {
        let tag_bits = r.read_bits(2)?;
        let tag = StoredBoardTag::from_bits(tag_bits).ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid board tag: {tag_bits:#3b}"),
        ))?;

        let board = match tag {
            StoredBoardTag::StartPosition => DEFAULT_BOARD,
            StoredBoardTag::Sparse => {
                let mut board = ChessBoard::empty();
                let piece_count: u8 = r.read_bits(6)?;
                for _ in 0..piece_count {
                    let square = r.read()?;
                    let color = r.read()?;
                    let piece = r.read()?;
                    board.set(piece, color, square);
                }
                board
            }
            StoredBoardTag::Dense => {
                let mut board = ChessBoard::empty();
                let occupied: BitBoard = r.read()?;
                for square in occupied {
                    let color = r.read()?;
                    let piece = r.read()?;
                    board.set(piece, color, square);
                }
                board
            }
            StoredBoardTag::DenseHuffman => {
                let mut board = ChessBoard::empty();
                for sq in Square::iter_bottom_top() {
                    let has_piece = r.read::<bool>()?;
                    if !has_piece {
                        continue;
                    }
                    let is_not_pawn = r.read::<bool>()?;
                    if is_not_pawn {
                        let piece_bits: u8 = r.read_bits(3)?;
                        let piece = match piece_bits {
                            0b000 => Piece::Knight,
                            0b001 => Piece::Bishop,
                            0b010 => Piece::Rook,
                            0b011 => Piece::Queen,
                            0b100 => Piece::King,
                            _ => {
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    format!("Invalid dense huffman piece bits: {piece_bits:#3b}"),
                                ));
                            }
                        };
                        let color = r.read()?;
                        board.set(piece, color, sq);
                    } else {
                        let color = r.read()?;
                        board.set(Piece::Pawn, color, sq);
                    }
                }
                board
            }
        };

        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::square::*;
    use std::str::FromStr;

    #[test]
    fn test_round_trip_start_position() {
        let original = ChessBoard::default();

        let stored = StoredBoard::new(&original).expect("Failed to encode default board");
        assert_eq!(stored.tag(), Some(StoredBoardTag::StartPosition));

        let restored = stored.restore().expect("Failed to decode default board");

        assert_eq!(
            original, restored,
            "Restored start position does not match original"
        );
    }

    #[test]
    fn test_round_trip_sparse_position() {
        let mut original = ChessBoard::empty();

        original.set(Piece::King, Color::White, E1);
        original.set(Piece::Rook, Color::White, A1);
        original.set(Piece::King, Color::Black, E8);

        assert!(original.total_piece_count() < 10, "Board should be sparse");

        let stored = StoredBoard::new(&original).expect("Failed to encode sparse board");
        assert_eq!(stored.tag(), Some(StoredBoardTag::Sparse));

        let restored = stored.restore().expect("Failed to decode sparse board");

        assert_eq!(
            original, restored,
            "Restored sparse position does not match original"
        );
    }

    #[test]
    fn test_round_trip_empty_position() {
        let original = ChessBoard::empty();

        let stored = StoredBoard::new(&original).expect("Failed to encode empty board");
        assert_eq!(stored.tag(), Some(StoredBoardTag::Sparse));

        let restored = stored.restore().expect("Failed to decode empty board");

        assert_eq!(
            original, restored,
            "Restored empty position does not match original"
        );
    }

    #[test]
    fn test_round_trip_dense_position() {
        let fen_str = "rnbqkbnr/8/8/8/8/8/8/RNBQKBNR";
        let original = ChessBoard::from_str(fen_str).expect("Failed to parse FEN");

        assert!(original.total_piece_count() >= 10, "Board should be dense");

        let stored = StoredBoard::new(&original).expect("Failed to encode dense board");
        assert_eq!(stored.tag(), Some(StoredBoardTag::Dense));

        let restored = stored.restore().expect("Failed to decode dense board");
        assert_eq!(
            original, restored,
            "Restored dense position does not match original"
        );
    }

    #[test]
    fn test_round_trip_dense_huffman_position() {
        let fen_str = "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R";
        let original = ChessBoard::from_str(fen_str).expect("Failed to parse FEN");

        let stored = StoredBoard::new(&original).expect("Failed to encode board");
        assert_eq!(stored.tag(), Some(StoredBoardTag::DenseHuffman));

        let restored = stored.restore().expect("Failed to decode board");
        assert_eq!(
            original, restored,
            "Restored dense huffman position does not match original"
        );
    }
}
