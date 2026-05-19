// ToDo: Make all states achievable in game
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum GameState {
    Running,
    Checkmate,
    Stalemate,
    DrawSeventyFive,
    DrawFiftyMoveClaimable,
    DrawRepetitionClaimable,
    DrawInsufficientMaterial,
}
