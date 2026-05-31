use crate::core::position::Position;
use crate::moves::naive::NaivePromotionMove;
use crate::prelude::ChessMove;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PuzzlePlayer {
    pz: Puzzle,
    index: usize,
    times_failed: usize,
}

impl PuzzlePlayer {
    pub fn new(pz: Puzzle) -> Self {
        Self {
            pz,
            index: 0,
            times_failed: 0,
        }
    }

    pub fn try_move(&mut self, mv: NaivePromotionMove) -> PuzzleEvent {
        let pos = self.position();
        let Some(chess_move) = mv.get_move(&pos) else {
            self.times_failed += 1;
            return PuzzleEvent::Illegal;
        };
        if chess_move != self.pz.solution[self.index] {
            self.times_failed += 1;
            return PuzzleEvent::Wrong;
        }
        self.index += 1;
        if self.index < self.pz.solution.len() {
            self.index += 1;
        }
        if self.index >= self.pz.solution.len() {
            PuzzleEvent::Solved
        } else {
            PuzzleEvent::Correct
        }
    }

    pub fn position(&self) -> Position {
        let mut position = self.pz.pos.make_move(self.pz.last_move);
        for &mv in &self.pz.solution[..self.index] {
            position = position.make_move(mv);
        }
        position
    }

    pub fn last_move(&self) -> Option<&ChessMove> {
        if self.index == 0 {
            Some(&self.pz.last_move)
        } else {
            self.pz.solution.get(self.index - 1)
        }
    }

    pub fn times_failed(&self) -> usize {
        self.times_failed
    }
}

pub enum PuzzleEvent {
    Illegal,
    Wrong,
    Correct,
    Solved,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "bit-codec",
    derive(bit_codec::BitEncode, bit_codec::BitDecode)
)]
pub struct Puzzle {
    pos: Position,
    last_move: ChessMove,
    solution: Vec<ChessMove>,
}

impl Puzzle {
    pub fn new(pos: Position, last_move: ChessMove, solution: Vec<ChessMove>) -> Self {
        Self {
            pos,
            last_move,
            solution,
        }
    }

    pub fn position(&self) -> &Position {
        &self.pos
    }

    pub fn last_move(&self) -> &ChessMove {
        &self.last_move
    }

    pub fn solution(&self) -> &[ChessMove] {
        &self.solution
    }
}
