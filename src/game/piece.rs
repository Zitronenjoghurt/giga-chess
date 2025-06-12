use crate::game::color::Color;

pub const PIECES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Piece {
    pub fn get_icon(&self, color: Color) -> &str {
        if color == Color::White {
            match self {
                Self::Pawn => "♙",
                Self::Knight => "♘",
                Self::Bishop => "♗",
                Self::Rook => "♖",
                Self::Queen => "♕",
                Self::King => "♔",
            }
        } else {
            match self {
                Self::Pawn => "♟",
                Self::Knight => "♞",
                Self::Bishop => "♝",
                Self::Rook => "♜",
                Self::Queen => "♛",
                Self::King => "♚",
            }
        }
    }
}
