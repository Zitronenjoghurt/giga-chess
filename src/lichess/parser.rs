use crate::lichess::entry::RawLichessPuzzleEntry;
use csv::DeserializeRecordsIntoIter;
use std::fs::File;
use std::io::BufReader;
use zstd::Decoder;

pub struct LichessPuzzleParser {
    iter: DeserializeRecordsIntoIter<Decoder<'static, BufReader<File>>, RawLichessPuzzleEntry>,
}

impl LichessPuzzleParser {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let decoder = Decoder::new(file)?;
        let csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(decoder);
        Ok(Self {
            iter: csv_reader.into_deserialize(),
        })
    }
}

impl Iterator for LichessPuzzleParser {
    type Item = Result<RawLichessPuzzleEntry, csv::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
