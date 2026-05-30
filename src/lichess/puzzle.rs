use crate::core::puzzle::Puzzle;
use crate::lichess::themes::LichessPuzzleTheme;
use crate::storage::io::{BitDecode, BitEncode, BitReader, BitWriter};
use std::io::{Read, Write};

pub struct LichessPuzzle {
    pub id: String,
    pub puzzle: Puzzle,
    pub themes: Vec<LichessPuzzleTheme>,
    pub rating: u16,
    pub rating_deviation: u16,
    pub times_played: u32,
}

impl BitEncode for LichessPuzzle {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<()> {
        w.write(&self.id)?;
        w.write(&self.puzzle)?;
        w.write(&self.themes)?;
        w.write_bits(self.rating, 13)?;
        w.write_bits(self.rating_deviation, 9)?;
        w.write(&self.times_played)?;
        Ok(())
    }
}

impl BitDecode for LichessPuzzle {
    fn decode<R: Read>(r: &mut BitReader<R>) -> std::io::Result<Self> {
        Ok(Self {
            id: r.read()?,
            puzzle: r.read()?,
            themes: r.read()?,
            rating: r.read_bits(13)?,
            rating_deviation: r.read_bits(9)?,
            times_played: r.read()?,
        })
    }
}
