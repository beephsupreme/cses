use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum CsesError {
    #[error("Invalid input => {0}")]
    InvalidInput(String),
    #[error("OutOfRange")]
    OutOfRange,
    #[error("cannot parse => {0}")]
    ParseError(String),
    #[error("expected{0}, received {1}")]
    InvalidSize(usize, usize),
    #[error("element too large ({0})")]
    Overflow(usize),
    #[error("element too small ({0})")]
    Underflow(usize),
    #[error("{0} unique found, {1}) expected")]
    DuplicateValues(usize, usize),
}