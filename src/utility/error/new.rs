use crate::{Error, DWORD, HRESULT, HRESULT_FROM_WIN32};

impl Error {
    /// Creates a new [`Error`] from an [`HRESULT`]
    pub const fn new(error: HRESULT) -> Self {
        Error(error)
    }

    /// Creates a new [`Error`] from a Win32 error
    pub const fn new_win32(error: DWORD) -> Self {
        Error(HRESULT_FROM_WIN32!(error))
    }
}
