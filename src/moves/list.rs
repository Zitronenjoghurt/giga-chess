use crate::prelude::ChessMove;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MoveList {
    #[cfg_attr(feature = "serde", serde(with = "crate::big_array"))]
    moves: [ChessMove; 256],
    len: u8,
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: [ChessMove::default(); 256],
            len: 0,
        }
    }
}

impl MoveList {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, mv: ChessMove) {
        self.moves[self.len as usize] = mv;
        self.len += 1;
    }

    pub(crate) fn extend(&mut self, other: &[ChessMove]) {
        self.moves[self.len as usize..self.len as usize + other.len()].copy_from_slice(other);
        self.len += other.len() as u8;
    }

    pub fn contains(&self, mv: ChessMove) -> bool {
        self.moves.contains(&mv)
    }

    pub fn as_slice(&self) -> &[ChessMove] {
        &self.moves[..self.len as usize]
    }

    pub fn iter(&self) -> impl Iterator<Item = &ChessMove> {
        self.as_slice().iter()
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl IntoIterator for MoveList {
    type Item = ChessMove;
    type IntoIter = std::iter::Take<std::array::IntoIter<ChessMove, 256>>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.into_iter().take(self.len as usize)
    }
}
