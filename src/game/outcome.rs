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
    /// One sides king was in check and had no legal moves to move it out of check.
    Checkmate,
    /// One side resigned.
    Resignation,
    /// One side ran out of time.
    Timeout,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DrawReason {
    /// One side had no legal moves but was not in check.
    Stalemate,
    /// Both sides agreed to a draw.
    Agreement,
    /// 50 consecutive moves without a pawn advance or capture and the draw was claimed.
    FiftyMoveRule,
    /// 75 consecutive moves without a pawn advance or capture.
    SeventyFiveMoveRule,
    /// A position was repeated 3 times and the draw was claimed.
    ThreefoldRepetition,
    /// A position was repeated 5 times.
    FivefoldRepetition,
    /// Both sides had insufficient material.
    InsufficientMaterial,
    /// One side ran out of time but the other side had insufficient material.
    TimeoutVsInsufficient,
}
