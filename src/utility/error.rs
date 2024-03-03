use crate::{
    string::wcslen, winsock2::WSAGetLastError, FormatMessage, GetLastError, LocalFree, DWORD,
    FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS,
    HRESULT, HRESULT_FROM_WIN32, LANG_NEUTRAL, LPWSTR, MAKELANGID, SUBLANG_DEFAULT,
};
use std::{
    fmt::Write,
    ptr::{null, null_mut},
};

/// A specialized result for Windows errors
pub type Result<T> = core::result::Result<T, Error>;

/// An error reported by Windows
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error(HRESULT);

/// Convert a Windows call result (0/[`null`] on error) into a [`Result<T>`]
#[macro_export]
macro_rules! try_get_last_error {
    ($expr: expr) => {{
        let result = unsafe { $expr };
        if result as usize == 0 {
            Err($crate::Error::get_last_error())
        } else {
            Ok(result)
        }
    }};
}

/// Convert a Windows Socket call result ([`SOCKET_ERROR`] on error) into a [`Result<T>`]
#[macro_export]
macro_rules! try_wsa_get_last_error {
    ($expr: expr) => {{
        let result = unsafe { $expr };
        if result == $crate::winsock2::SOCKET_ERROR {
            Err($crate::Error::wsa_get_last_error())
        } else {
            Ok(result)
        }
    }};
}

impl Error {
    /// Creates a new [`Error`] from an [`HRESULT`]
    pub const fn new(error: HRESULT) -> Self {
        Error(error)
    }

    /// Creates a new [`Error`] from a Win32 error
    pub const fn new_win32(error: DWORD) -> Self {
        Error(HRESULT_FROM_WIN32!(error))
    }

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

    /// Gets the underlying error value
    pub fn get(&self) -> HRESULT {
        self.0
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Create the message
        let mut buffer: LPWSTR = null_mut();
        unsafe {
            FormatMessage(
                FORMAT_MESSAGE_ALLOCATE_BUFFER
                    | FORMAT_MESSAGE_FROM_SYSTEM
                    | FORMAT_MESSAGE_IGNORE_INSERTS,
                null(),
                self.0 as DWORD,
                MAKELANGID!(LANG_NEUTRAL, SUBLANG_DEFAULT),
                &mut buffer as *mut LPWSTR as _,
                0,
                null_mut(),
            )
        };

        // Verify there is a message
        if buffer == null_mut() {
            return write!(f, "unknown error {:#X}", self.0);
        }

        // Get the length and convert the buffer to a slice
        let length = unsafe { wcslen(buffer) };
        let slice = unsafe { std::slice::from_raw_parts_mut(buffer, length) };

        // Print the message
        let mut result = Ok(());
        for c in char::decode_utf16(slice.into_iter().map(|val| *val)) {
            let c = c.unwrap_or(char::REPLACEMENT_CHARACTER);
            if c == '\n' || c == '\r' {
                break;
            }

            result = f.write_char(c);
            if result.is_err() {
                break;
            }
        }

        // Free the message
        unsafe { LocalFree(buffer.cast()) };

        result
    }
}
