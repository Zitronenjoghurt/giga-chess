use std::error::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct CastlingRights {
    pub white_king_side: bool,
    pub white_queen_side: bool,
    pub black_king_side: bool,
    pub black_queen_side: bool,
}

impl CastlingRights {
    pub fn none() -> Self {
        Self {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }

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
