use crate::core::puzzle::Puzzle;
use crate::lichess::parser::LichessPuzzleEntry;
use crate::lichess::puzzle::LichessPuzzle;
use crate::storage::io::{BitDecode, BitReader, BitWriter};

#[derive(Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LichessPuzzleArchive(Vec<u8>);

impl LichessPuzzleArchive {
    pub fn try_from_iter<I>(iter: I) -> std::io::Result<Self>
    where
        I: IntoIterator<Item = std::io::Result<LichessPuzzleEntry>>,
    {
        let mut buffer = Vec::new();
        {
            let encoder = zstd::Encoder::new(&mut buffer, 22)?;
            let mut writer = BitWriter::new(encoder);

            for entry in iter {
                let entry = entry?;
                let themes = entry
                    .themes
                    .iter()
                    .map(|t| {
                        t.parse()
                            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let puzzle = LichessPuzzle {
                    id: entry.id,
                    puzzle: Puzzle::new(entry.pos, entry.moves),
                    themes,
                    rating: entry.rating as u16,
                    rating_deviation: entry.rating_deviation as u16,
                    times_played: entry.times_played as u32,
                };
                writer.write(&puzzle)?;
            }

            writer.flush()?;
            writer.into_inner().finish()?;
        }

        Ok(Self(buffer))
    }

    pub fn iter(&self) -> std::io::Result<LichessPuzzleIter<'_>> {
        let decoder = zstd::Decoder::new(self.0.as_slice())?;
        Ok(LichessPuzzleIter {
            reader: BitReader::new(decoder),
        })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

pub struct LichessPuzzleIter<'a> {
    reader: BitReader<zstd::Decoder<'a, std::io::BufReader<&'a [u8]>>>,
}

impl<'a> Iterator for LichessPuzzleIter<'a> {
    type Item = std::io::Result<LichessPuzzle>;

    fn next(&mut self) -> Option<Self::Item> {
        match LichessPuzzle::decode(&mut self.reader) {
            Ok(puzzle) => Some(Ok(puzzle)),
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}
