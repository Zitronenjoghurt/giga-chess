use crate::core::square::*;
use crate::error::{FenError, FenResult};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "bit-codec",
    derive(bit_codec::BitEncode, bit_codec::BitDecode)
)]
/// Contains information on who is still allowed to castle and in which direction.
pub struct CastlingRights {
    /// If white is allowed to castle king side.
    pub white_king_side: bool,
    /// If white is allowed to castle queen side.
    pub white_queen_side: bool,
    /// If black is allowed to castle king side.
    pub black_king_side: bool,
    /// If black is allowed to castle queen side.
    pub black_queen_side: bool,
}

impl CastlingRights {
    /// Create new [`CastlingRights`] where no castling is allowed.
    ///
    /// Returns: [`CastlingRights`]
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::core::castling::CastlingRights;
    ///
    /// let rights = CastlingRights::none();
    ///
    /// assert!(!rights.white_king_side);
    /// assert!(!rights.white_queen_side);
    /// assert!(!rights.black_king_side);
    /// assert!(!rights.black_queen_side);
    /// ```
    pub fn none() -> Self {
        Self {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }

    pub fn update(&mut self, from: Square, to: Square) {
        self.white_king_side &= from != E1;
        self.white_queen_side &= from != E1;
        self.black_king_side &= from != E8;
        self.black_queen_side &= from != E8;

        self.white_king_side &= from != H1 && to != H1;
        self.white_queen_side &= from != A1 && to != A1;
        self.black_king_side &= from != H8 && to != H8;
        self.black_queen_side &= from != A8 && to != A8;
    }

    pub fn bits(&self) -> u8 {
        (self.white_king_side as u8)
            | (self.white_queen_side as u8) << 1
            | (self.black_king_side as u8) << 2
            | (self.black_queen_side as u8) << 3
    }

    pub fn from_bits(bits: u8) -> Self {
        Self {
            white_king_side: (bits & 0b0001) != 0,
            white_queen_side: (bits & 0b0010) != 0,
            black_king_side: (bits & 0b0100) != 0,
            black_queen_side: (bits & 0b1000) != 0,
        }
    }
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}

impl FromStr for CastlingRights {
    type Err = FenError;

    fn from_str(s: &str) -> FenResult<Self> {
        if s == "-" {
            return Ok(Self::none());
        }

        let mut rights = Self::default();
        for c in s.chars() {
            match c {
                'K' => rights.white_king_side = true,
                'Q' => rights.white_queen_side = true,
                'k' => rights.black_king_side = true,
                'q' => rights.black_queen_side = true,
                _ => return Err(FenError::InvalidCastlingRights(s.to_string())),
            }
        }

        Ok(rights)
    }
}

impl Display for CastlingRights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fen = String::new();
        if self.white_king_side {
            fen.push('K');
        }
        if self.white_queen_side {
            fen.push('Q');
        }
        if self.black_king_side {
            fen.push('k');
        }
        if self.black_queen_side {
            fen.push('q');
        }
        if fen.is_empty() {
            write!(f, "-")
        } else {
            write!(f, "{fen}")
        }
    }
}
