#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Running = 0,
    Stalemate = 1,
    Checkmate = 2,
    DrawFiftyMove = 3,
}

impl GameStatus {
    pub fn is_draw(&self) -> bool {
        match self {
            Self::DrawFiftyMove | Self::Stalemate => true,
            Self::Running | Self::Checkmate => false,
        }
    }
}
