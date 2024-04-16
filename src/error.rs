use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum LibraryError {
    #[error("{0}")]
    InvalidInput(String),
    #[error("{0}, {1}, {2}")]
    OutOfRange(String, String, String),
    #[error("{0}")]
    ParseError(String),
    #[error("expected {0}, found: {1}")]
    SizeMismatch(usize, usize),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
