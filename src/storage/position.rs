use crate::core::position::Position;
use crate::core::zobrist::ZobristKeys;
use crate::prelude::{CastlingRights, ChessBoard, Color};
use crate::storage::io::{BitDecode, BitEncode, BitReader, BitWriter};
use std::io::{Read, Write};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StoredPosition(Vec<u8>);

impl StoredPosition {
    pub fn new(board: &Position) -> std::io::Result<Self> {
        let mut buffer = Vec::new();
        {
            let mut writer = BitWriter::new(&mut buffer);
            board.encode(&mut writer)?;
            writer.flush()?;
        }
        Ok(Self(buffer))
    }

    pub fn restore(self) -> std::io::Result<Position> {
        let mut reader = BitReader::new(self.0.as_slice());
        Position::decode(&mut reader)
    }
}

impl BitEncode for Position {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<()> {
        w.write(&self.board)?;
        w.write(&(bool::from(self.side_to_move)))?;
        w.write(&self.castling_rights)?;
        w.write(&self.en_passant_square)?;
        w.write_bits(self.half_moves, 8)?;
        w.write_bits(self.full_moves, 13)?;
        Ok(())
    }
}

impl BitDecode for Position {
    fn decode<R: Read>(r: &mut BitReader<R>) -> std::io::Result<Self> {
        let board = r.read::<ChessBoard>()?;
        let side_to_move = Color::from(r.read::<bool>()?);
        let castling_rights = r.read::<CastlingRights>()?;
        let en_passant_square = r.read()?;
        let half_moves = r.read_bits(8)?;
        let full_moves = r.read_bits(13)?;
        let pos = Self {
            board,
            side_to_move,
            castling_rights,
            en_passant_square,
            half_moves,
            full_moves,
            hash: 0,
        };
        Ok(Self {
            hash: ZobristKeys::full_hash(&pos),
            ..pos
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::position::Position;
    use std::str::FromStr;

    #[test]
    fn test_round_trip_start_position() {
        let original = Position::default();

        let stored = StoredPosition::new(&original).expect("Failed to encode default position");
        let restored = stored.restore().expect("Failed to decode default position");

        assert_eq!(
            original, restored,
            "Restored start position does not match original"
        );
    }

    #[test]
    fn test_round_trip_with_en_passant() {
        let fen_str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 2";
        let original = Position::from_str(fen_str).expect("Failed to parse FEN");

        let stored = StoredPosition::new(&original).expect("Failed to encode en passant position");
        let restored = stored
            .restore()
            .expect("Failed to decode en passant position");

        assert_eq!(
            original, restored,
            "Restored en passant position does not match original"
        );
    }

    #[test]
    fn test_round_trip_no_castling_late_game() {
        let fen_str = "8/8/8/4k3/8/8/4K3/8 w - - 50 45";
        let original = Position::from_str(fen_str).expect("Failed to parse FEN");

        let stored = StoredPosition::new(&original).expect("Failed to encode endgame position");
        let restored = stored.restore().expect("Failed to decode endgame position");

        assert_eq!(
            original, restored,
            "Restored endgame position does not match original"
        );
    }

    #[test]
    fn test_round_trip_complex_middle_game() {
        let fen_str = "r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R w Kq - 4 17";
        let original = Position::from_str(fen_str).expect("Failed to parse FEN");

        let stored = StoredPosition::new(&original).expect("Failed to encode middle game position");
        let restored = stored
            .restore()
            .expect("Failed to decode middle game position");

        assert_eq!(
            original, restored,
            "Restored middle game position does not match original"
        );
    }
}
