use crate::core::puzzle::Puzzle;
use crate::lichess::themes::LichessPuzzleTheme;

#[cfg_attr(
    feature = "bit-codec",
    derive(bit_codec::BitEncode, bit_codec::BitDecode)
)]
pub struct LichessPuzzle {
    pub id: String,
    pub puzzle: Puzzle,
    pub themes: Vec<LichessPuzzleTheme>,
    #[cfg_attr(feature = "bit-codec", bits(13))]
    pub rating: u16,
    #[cfg_attr(feature = "bit-codec", bits(9))]
    pub rating_deviation: u16,
    pub times_played: u32,
}
