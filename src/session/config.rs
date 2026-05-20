use crate::game::mode::GameMode;
use crate::notation::pgn::PgnHeaders;
use crate::session::clock::ChessClockConfig;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SessionConfig {
    pub mode: GameMode,
    pub starting_position: StartingPosition,
    pub time_control: TimeControl,
    pub pgn: PgnHeaders,
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StartingPosition {
    #[default]
    Default,
    Fen(String),
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TimeControl {
    #[default]
    Unlimited,
    Clock(ChessClockConfig),
}
