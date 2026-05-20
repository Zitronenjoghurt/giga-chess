use crate::prelude::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GameOutcome {
    Decisive {
        winner: Color,
        reason: DecisiveReason,
    },
    Draw(DrawReason),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DecisiveReason {
    Checkmate,
    Resignation,
    Timeout,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DrawReason {
    Stalemate,
    Agreement,
    Timeout,
    FiftyMoveRule,
    SeventyFiveMoveRule,
    ThreefoldRepetition,
    FivefoldRepetition,
    InsufficientMaterial,
    TimeoutVsInsufficient,
}
