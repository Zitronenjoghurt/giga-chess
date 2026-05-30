use crate::core::position::Position;
use crate::moves::naive::NaivePromotionMove;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Puzzle {
    pos: Position,
    moves: Vec<NaivePromotionMove>,
}

impl Puzzle {
    pub fn new(pos: Position, moves: Vec<NaivePromotionMove>) -> Self {
        Self { pos, moves }
    }

    pub fn position(&self) -> &Position {
        &self.pos
    }

    pub fn moves(&self) -> &[NaivePromotionMove] {
        &self.moves
    }
}
