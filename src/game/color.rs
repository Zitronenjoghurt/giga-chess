use std::error::Error;

pub const COLORS: [Color; 2] = [Color::White, Color::Black];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn get_fen_char(&self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
}

impl TryFrom<&str> for Color {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "W" | "w" => Ok(Color::White),
            "B" | "b" => Ok(Color::Black),
            _ => Err(format!("Invalid color '{value}'").into()),
        }
    }
}
