pub const COLORS: [Color; 2] = [Color::White, Color::Black];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White = 0,
    Black = 1,
}
