use std::error::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    /// returns: [`CastlingRights`]
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::game::castling_rights::CastlingRights;
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

    /// Formats available castling rights FEN-compliant.
    ///
    /// returns: String
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::game::castling_rights::CastlingRights;
    ///
    /// let rights1 = CastlingRights::default();
    /// assert_eq!(rights1.get_fen_string(), "KQkq");
    ///
    /// let rights2 = CastlingRights::none();
    /// assert_eq!(rights2.get_fen_string(), "-");
    ///
    /// let rights3 = CastlingRights {
    ///     white_king_side: true,
    ///     white_queen_side: false,
    ///     black_king_side: false,
    ///     black_queen_side: true,
    /// };
    /// assert_eq!(rights3.get_fen_string(), "Kq");
    /// ```
    pub fn get_fen_string(&self) -> String {
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

        if fen.is_empty() { "-".to_string() } else { fen }
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

impl TryFrom<&str> for CastlingRights {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "-" {
            return Ok(Self::none());
        }

        let mut rights = Self::default();
        for c in value.chars() {
            match c {
                'K' => rights.white_king_side = true,
                'Q' => rights.white_queen_side = true,
                'k' => rights.black_king_side = true,
                'q' => rights.black_queen_side = true,
                _ => return Err(format!("Invalid character '{}'", value).into()),
            }
        }

        Ok(rights)
    }
}
