pub type SfResult<T> = Result<T, SfError>;

#[derive(Debug, thiserror::Error)]
pub enum SfError {
    #[error("Missing expected token/terminator: {0}")]
    MissingToken(String),
    #[error("Parse failed: {0}")]
    ParseFailed(String),
    #[error("Unexpected EOF")]
    UnexpectedEof,
    #[error("Unexpected token: expected {expected}, got {got}")]
    UnexpectedToken { expected: String, got: String },
    #[error("Unknown event of type {event_type}: {data}")]
    UnknownEvent { event_type: String, data: String },
    #[error("Invalid id value of type {id_type}: {value}")]
    UnknownIdValue { id_type: String, value: String },
    #[error("Invalid option value of type {value_type}: {value}")]
    UnknownOptionValue { value_type: String, value: String },
    #[error("Unknown score type: {0}")]
    UnknownScoreType(String),
}
