use crate::prelude::Square;

#[derive(Debug, serde::Deserialize)]
pub struct RawLichessPuzzleEntry {
    pub id: String,
    pub fen: String,
    pub moves: String,
    pub rating: usize,
    pub rating_deviation: usize,
    pub times_played: usize,
    pub themes: String,
    pub game_url: String,
    pub opening_tags: String,
}

#[derive(Debug)]
pub struct LichessPuzzleEntry {
    pub id: String,
    pub fen: String,
    pub moves: Vec<(Square, Square)>,
    pub rating: usize,
    pub rating_deviation: usize,
    pub times_played: usize,
    pub themes: Vec<String>,
    pub game_url: String,
    pub opening_tags: Vec<String>,
}
