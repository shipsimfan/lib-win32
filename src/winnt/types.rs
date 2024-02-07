use crate::WORD;
use std::ffi::{c_long, c_void};

/// A handle to an object
pub type HANDLE = PVOID;

/// The return codes used by COM interfaces
pub type HRESULT = LONG;

/// A language identifier
pub type LANGID = WORD;

/// A 64-bit signed integer. The range is -9223372036854775808 through 9223372036854775807 decimal.
pub type LONG = c_long;

/// An [`LPWSTR`]
pub type LPTSTR = LPWSTR;

/// A pointer to a null-terminated string of 16-bit Unicode characters
pub type LPWSTR = *mut WCHAR;

/// A pointer to any type
pub type PVOID = *mut c_void;

/// A [`WCHAR`]
pub type TCHAR = WCHAR;

/// A 16-bit Unicode character
pub type WCHAR = u16;
