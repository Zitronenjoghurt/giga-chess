use crate::game::color::Color;
use std::error::Error;

pub const PIECES: [Piece; 6] = [
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

    pub fn get_char(&self) -> char {
        match self {
            Self::Pawn => 'P',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Rook => 'R',
            Self::Queen => 'Q',
            Self::King => 'K',
        }
    }

    pub fn get_fen_char(&self, color: Color) -> char {
        if color == Color::White {
            self.get_char()
        } else {
            self.get_char().to_ascii_lowercase()
        }
    }
}

impl TryFrom<char> for Piece {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'P' | 'p' => Ok(Self::Pawn),
            'N' | 'n' => Ok(Self::Knight),
            'B' | 'b' => Ok(Self::Bishop),
            'R' | 'r' => Ok(Self::Rook),
            'Q' | 'q' => Ok(Self::Queen),
            'K' | 'k' => Ok(Self::King),
            _ => Err(format!("Invalid piece character '{value}'").into()),
        }
    }
}
