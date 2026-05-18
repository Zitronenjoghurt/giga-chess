pub type ChessResult<T> = Result<T, ChessError>;

#[derive(Debug, thiserror::Error)]
pub enum ChessError {
    #[error("Invalid castling rights: {0}")]
    InvalidCastlingRights(String),
    #[error("Invalid chess board: {0}")]
    InvalidChessBoard(String),
    #[error("Invalid color: {0}")]
    InvalidColor(String),
    #[error("Invalid piece: {0}")]
    InvalidPiece(String),
    #[error("Invalid position: {0}")]
    InvalidPosition(String),
    #[error("Invalid square: {0}")]
    InvalidSquare(String),
}
