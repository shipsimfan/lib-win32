use crate::{
    raw::{DWord, GetLastError, HResult, WSAGetLastError},
    String, HRESULT_FROM_WIN32,
};
use std::ptr::{null, null_mut};

/// Win32 Error
pub struct Error(HResult);

/// Converts `error` into a Windows string using `FormatMessageW`
pub(super) fn string_from_error(error: HResult) -> Option<String> {
    use crate::{
        raw::{
            FormatMessageW, LPWStr, FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_FROM_SYSTEM,
            FORMAT_MESSAGE_IGNORE_INSERTS, LANG_NEUTRAL, SUBLANG_DEFAULT,
        },
        MAKELANGID,
    };

    const FLAGS: DWord =
        FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS;
    const LANGUAGE_ID: DWord = MAKELANGID!(LANG_NEUTRAL, SUBLANG_DEFAULT) as DWord;

    if error == 0 {
        return None;
    }

    let mut message_ptr: LPWStr = null_mut();
    let buffer = (&mut message_ptr) as *mut LPWStr as LPWStr;
    let size = unsafe {
        FormatMessageW(
            FLAGS,
            null_mut(),
            error as DWord,
            LANGUAGE_ID,
            buffer,
            0,
            null(),
        )
    };

    if size == 0 {
        return None;
    }

    let true_size = size as usize + 1; // Returned size excludes the null character
    let string = unsafe { String::from_raw_parts(message_ptr, true_size, true_size) };

    Some(string)
}

impl Error {
    /// Create an error from an [`HResult`]
    pub fn new(hresult: HResult) -> Self {
        Error(hresult)
    }

    /// Create an error from a Win32 error
    pub fn win32(error: DWord) -> Self {
        Error(HRESULT_FROM_WIN32!(error))
    }

    /// Create an error using [`GetLastError`]
    pub fn last_error() -> Self {
        Error::win32(unsafe { GetLastError() })
    }

    /// Create an error using [`WSAGetLastError`]
    pub fn wsa_last_error() -> Self {
        Error::new(unsafe { WSAGetLastError() })
    }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match string_from_error(self.0) {
            Some(message) => message.fmt(f),
            None => write!(f, "unknown error (0x{:08X})", self.0),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
