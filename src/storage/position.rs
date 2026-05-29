use crate::core::position::Position;
use crate::prelude::DEFAULT_BOARD;

#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StoredRawPosition {
    StartingPosition,
}

impl StoredRawPosition {
    pub fn new(position: &Position) -> Self {
        if position.board == DEFAULT_BOARD && position.full_moves == 1 {
            return Self::StartingPosition;
        };
        todo!()
    }

    pub fn load(&self) -> Position {
        match self {
            Self::StartingPosition => Position::default(),
        }
    }
}
