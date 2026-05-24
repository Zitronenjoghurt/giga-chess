use crate::core::square::*;
use crate::error::{FenError, FenResult};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumIter, strum::EnumIs, strum::EnumCount, strum::FromRepr)
)]
#[repr(u8)]
pub enum Piece {
    Pawn = 0,
    Knight = 1,
    Bishop = 2,
    Rook = 3,
    Queen = 4,
    King = 5,
}

impl Piece {
    pub const ALL: [Self; 6] = [
        Self::Pawn,
        Self::Knight,
        Self::Bishop,
        Self::Rook,
        Self::Queen,
        Self::King,
    ];

    pub fn icon(&self, color: Color) -> &str {
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

    pub fn char(&self) -> char {
        match self {
            Self::Pawn => 'P',
            Self::Knight => 'N',
            Self::Bishop => 'B',
            Self::Rook => 'R',
            Self::Queen => 'Q',
            Self::King => 'K',
        }
    }

    pub fn fen_char(&self, color: Color) -> char {
        if color == Color::White {
            self.char()
        } else {
            self.char().to_ascii_lowercase()
        }
    }
}

impl FromStr for Piece {
    type Err = FenError;

    fn from_str(s: &str) -> FenResult<Self> {
        match s {
            "P" | "p" | "♙" | "♟" => Ok(Self::Pawn),
            "N" | "n" | "♘" | "♞" => Ok(Self::Knight),
            "B" | "b" | "♗" | "♝" => Ok(Self::Bishop),
            "R" | "r" | "♖" | "♜" => Ok(Self::Rook),
            "Q" | "q" | "♕" | "♛" => Ok(Self::Queen),
            "K" | "k" | "♔" | "♚" => Ok(Self::King),
            _ => Err(FenError::InvalidPiece(s.to_string())),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.char())
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "strum",
    derive(strum::EnumIter, strum::EnumIs, strum::EnumCount, strum::FromRepr)
)]
#[repr(u8)]
pub enum Color {
    #[default]
    White = 0,
    Black = 1,
}

impl Color {
    pub const ALL: [Color; 2] = [Color::White, Color::Black];

    #[cfg(feature = "rand")]
    pub fn random() -> Self {
        use rand::prelude::*;
        let mut rng = rand::rng();
        *Self::ALL.choose(&mut rng).unwrap_or(&Color::White)
    }

    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub const fn kingside_castle_squares(self) -> (Square, Square, Square, Square) {
        match self {
            Color::White => (E1, G1, H1, F1),
            Color::Black => (E8, G8, H8, F8),
        }
    }

    pub const fn queenside_castle_squares(self) -> (Square, Square, Square, Square) {
        match self {
            Color::White => (E1, C1, A1, D1),
            Color::Black => (E8, C8, A8, D8),
        }
    }
}

impl FromStr for Color {
    type Err = FenError;

    fn from_str(s: &str) -> FenResult<Self> {
        match s {
            "W" | "w" => Ok(Color::White),
            "B" | "b" => Ok(Color::Black),
            _ => Err(FenError::InvalidColor(s.to_string())),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "w"),
            Color::Black => write!(f, "b"),
        }
    }
}
