use crate::core::position::Position;
use crate::moves::naive::NaivePromotionMove;
use csv::DeserializeRecordsIntoIter;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{BufReader, Read};
use zstd::Decoder;

pub struct LichessPuzzleParser<R: Read> {
    inner: CsvParser<Decoder<'static, BufReader<R>>, RawLichessPuzzleEntry>,
}

impl<R: Read> LichessPuzzleParser<R> {
    pub fn new(reader: R) -> Result<Self, std::io::Error> {
        let decoder = Decoder::new(reader)?;
        Ok(Self {
            inner: CsvParser::new(decoder),
        })
    }
}

impl LichessPuzzleParser<File> {
    pub fn from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        Ok(Self::new(file)?)
    }
}

impl<R: Read> Iterator for LichessPuzzleParser<R> {
    type Item = std::io::Result<LichessPuzzleEntry>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|result| {
            let raw = result?;
            raw.try_into()
        })
    }
}

pub struct CsvParser<R: Read, T> {
    iter: DeserializeRecordsIntoIter<R, T>,
}

impl<R: Read, T: DeserializeOwned> CsvParser<R, T> {
    pub fn new(reader: R) -> Self {
        let csv_reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(reader);
        Self {
            iter: csv_reader.into_deserialize(),
        }
    }
}

impl<R: Read, T: DeserializeOwned> Iterator for CsvParser<R, T> {
    type Item = Result<T, csv::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct RawLichessPuzzleEntry {
    #[serde(rename = "PuzzleId")]
    pub id: String,
    #[serde(rename = "FEN")]
    pub fen: String,
    #[serde(rename = "Moves")]
    pub moves: String,
    #[serde(rename = "Rating")]
    pub rating: usize,
    #[serde(rename = "RatingDeviation")]
    pub rating_deviation: usize,
    #[serde(rename = "NbPlays")]
    pub times_played: usize,
    #[serde(rename = "Themes")]
    pub themes: String,
    #[serde(rename = "GameUrl")]
    pub game_url: String,
    #[serde(rename = "OpeningTags")]
    pub opening_tags: String,
}

#[derive(Debug)]
pub struct LichessPuzzleEntry {
    pub id: String,
    pub pos: Position,
    pub moves: Vec<NaivePromotionMove>,
    pub rating: usize,
    pub rating_deviation: usize,
    pub times_played: usize,
    pub themes: Vec<String>,
    pub game_url: String,
    pub opening_tags: Vec<String>,
}

impl TryFrom<RawLichessPuzzleEntry> for LichessPuzzleEntry {
    type Error = std::io::Error;

    fn try_from(raw: RawLichessPuzzleEntry) -> std::io::Result<Self> {
        let pos = raw
            .fen
            .parse::<Position>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let moves = raw
            .moves
            .split_whitespace()
            .map(|m| m.parse::<NaivePromotionMove>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        let themes = raw.themes.split_whitespace().map(String::from).collect();
        let opening_tags = raw
            .opening_tags
            .split_whitespace()
            .map(String::from)
            .collect();

        Ok(Self {
            id: raw.id,
            pos,
            moves,
            rating: raw.rating,
            rating_deviation: raw.rating_deviation,
            times_played: raw.times_played,
            themes,
            game_url: raw.game_url,
            opening_tags,
        })
    }
}
