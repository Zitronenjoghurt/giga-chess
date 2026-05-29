use crate::prelude::Square;
use crate::storage::io::{BitDecode, BitEncode, BitReader, BitWriter};
use std::io::Write;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StoredNaiveMoves(Vec<u8>);

impl StoredNaiveMoves {
    pub fn new(moves: &[(Square, Square)]) -> std::io::Result<Self> {
        let mut buffer = Vec::new();
        {
            let mut writer = BitWriter::new(&mut buffer);
            moves.encode(&mut writer)?;
            writer.flush()?;
        }
        Ok(Self(buffer))
    }

    pub fn restore(self) -> std::io::Result<Vec<(Square, Square)>> {
        let mut reader = BitReader::new(&self.0);
        BitDecode::decode(&mut reader)
    }
}

impl BitEncode for [(Square, Square)] {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<()> {
        w.write_u16(self.len() as u16)?;
        for (from, to) in self {
            w.write_bits(from.index(), 6)?;
            w.write_bits(to.index(), 6)?;
        }
        Ok(())
    }
}

impl BitDecode for Vec<(Square, Square)> {
    fn decode(r: &mut BitReader) -> std::io::Result<Self> {
        let len = r.read_u16()?;
        let mut moves = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let from = Square::new(r.read_bits(6)?);
            let to = Square::new(r.read_bits(6)?);
            moves.push((from, to));
        }
        Ok(moves)
    }
}
