pub use std::format as f;

pub use crate::error::LibraryError;

pub type Result<T> = core::result::Result<T, LibraryError>;
