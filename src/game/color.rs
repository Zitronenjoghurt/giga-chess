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
}
