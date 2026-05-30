use crate::core::puzzle::Puzzle;
use crate::storage::io::{BitDecode, BitEncode, BitReader, BitWriter};
use std::io::{Read, Write};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StoredPuzzle(Vec<u8>);

impl StoredPuzzle {
    pub fn new(puzzle: &Puzzle) -> std::io::Result<Self> {
        let mut buffer = Vec::new();
        {
            let mut writer = BitWriter::new(&mut buffer);
            puzzle.encode(&mut writer)?;
            writer.flush()?;
        }
        Ok(Self(buffer))
    }

    pub fn restore(self) -> std::io::Result<Puzzle> {
        let mut reader = BitReader::new(self.0.as_slice());
        Puzzle::decode(&mut reader)
    }
}

impl BitEncode for Puzzle {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<()> {
        w.write(self.position())?;
        w.write(self.moves())?;
        Ok(())
    }
}

impl BitDecode for Puzzle {
    fn decode<R: Read>(r: &mut BitReader<R>) -> std::io::Result<Self> {
        let pos = r.read()?;
        let moves = r.read()?;
        Ok(Self::new(pos, moves))
    }
}
