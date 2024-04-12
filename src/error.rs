use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("Invalid input => {0}")]
    InvalidInput(String),
    #[error("OutOfRange => {0}")]
    OutOfRange(String),
    #[error("cannot parse => {0}")]
    ParseError(String),
    #[error("expected{0}, actual {1}")]
    SizeMismatch(usize, usize),
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
