use crate::error::FenError;
use crate::prelude::{Piece, Square};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NaiveMove {
    pub from: Square,
    pub to: Square,
}

impl NaiveMove {
    pub fn new(from: Square, to: Square) -> Self {
        Self { from, to }
    }
}

impl FromStr for NaiveMove {
    type Err = FenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let from = s
            .get(..2)
            .ok_or_else(|| FenError::InvalidMove(format!("Invalid move: {s}")))?
            .parse()?;
        let to = s
            .get(2..4)
            .ok_or_else(|| FenError::InvalidMove(format!("Invalid move: {s}")))?
            .parse()?;
        Ok(Self { from, to })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NaivePromotionMove {
    pub mv: NaiveMove,
    pub promotion: Option<Piece>,
}

impl FromStr for NaivePromotionMove {
    type Err = FenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mv: NaiveMove = s.parse()?;
        let promotion = s
            .get(4..5)
            .map(|c| match c {
                "q" => Ok(Piece::Queen),
                "r" => Ok(Piece::Rook),
                "b" => Ok(Piece::Bishop),
                "n" => Ok(Piece::Knight),
                _ => Err(FenError::InvalidMove(format!("Invalid promotion: {s}"))),
            })
            .transpose()?;
        Ok(Self { mv, promotion })
    }
}
