pub use std::format as f;

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;
