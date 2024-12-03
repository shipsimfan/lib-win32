use crate::HRESULT;

mod display;
mod last_error;
mod macros;
mod new;

/// A specialized result for Windows errors
pub type Result<T> = core::result::Result<T, Error>;

/// An error reported by Windows
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error(pub HRESULT);

impl std::error::Error for Error {}
