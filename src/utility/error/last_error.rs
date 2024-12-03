use crate::{winsock2::WSAGetLastError, Error, GetLastError};

impl Error {
    /// Creates a new [`Error`] from the value of [`GetLastError`]
    pub fn get_last_error() -> Self {
        let error = unsafe { GetLastError() };
        Error::new_win32(error)
    }

    /// Creates a new [`Error`] from the value of [`WSAGetLastError`]
    pub fn wsa_get_last_error() -> Self {
        let error = unsafe { WSAGetLastError() };
        Error::new_win32(error as _)
    }
}
