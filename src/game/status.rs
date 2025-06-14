#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    Running = 0,
    Stalemate = 1,
    Checkmate = 2,
    DrawFiftyMove = 3,
}
