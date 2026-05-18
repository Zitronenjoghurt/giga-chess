use crate::prelude::ChessMove;

pub struct MoveList {
    moves: [ChessMove; 256],
    len: u8,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [ChessMove::default(); 256],
            len: 0,
        }
    }

    pub fn push(&mut self, mv: ChessMove) {
        self.moves[self.len as usize] = mv;
        self.len += 1;
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
}
