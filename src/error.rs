pub type FenResult<T> = Result<T, FenError>;
#[derive(Debug, thiserror::Error)]
pub enum FenError {
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

pub type ChessResult<T> = Result<T, ChessError>;
#[derive(Debug, thiserror::Error)]
pub enum ChessError {
    #[error("Illegal move")]
    IllegalMove,
    #[error("Illegal move in sequence at index {0}")]
    IllegalMoveSequence(usize),
    #[error("There is no draw to claim")]
    NoDrawClaimable,
}

pub type SessionResult<T> = Result<T, SessionError>;
#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error(transparent)]
    Chess(#[from] ChessError),
    #[error("Draw offer already offered")]
    DrawAlreadyOffered,
    #[error("Game is already over")]
    GameOver,
    #[error("Invalid FEN: {0}")]
    InvalidFen(#[from] FenError),
    #[error("There is no draw offer to accept or decline")]
    NoDrawOffer,
    #[error("Color is not to move")]
    NotMovingColor,
}
