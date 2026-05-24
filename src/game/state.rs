#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumIter, strum::EnumIs, strum::EnumCount)
)]
pub enum GameState {
    Running,
    Checkmate,
    Stalemate,
    DrawSeventyFive,
    DrawFivefold,
    DrawFiftyMoveClaimable,
    DrawRepetitionClaimable,
    DrawInsufficientMaterial,
}
