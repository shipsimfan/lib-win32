use crate::{ACL, SID, WORD};
use std::ffi::{c_long, c_void};

/// A handle to an object
pub type HANDLE = PVOID;

/// The return codes used by COM interfaces
pub type HRESULT = LONG;

/// A language identifier
pub type LANGID = WORD;

/// A 64-bit signed integer. The range is -9223372036854775808 through 9223372036854775807 decimal.
pub type LONG = c_long;

/// A pointer to a constant null-terminated string of 16-bit Unicode characters
pub type LPCWSTR = *const WCHAR;

/// An [`LPWSTR`]
pub type LPTSTR = LPWSTR;

/// A pointer to a null-terminated string of 16-bit Unicode characters
pub type LPWSTR = *mut WCHAR;

/// A pointer to an [`ACL`]
pub type PACL = *mut ACL;

/// A pointer to a [`SID`] structure
pub type PSID = *mut SID;

/// A pointer to any type
pub type PVOID = *mut c_void;

/// The [`SECURITY_DESCRIPTOR_CONTROL`] data type is a set of bit flags that qualify the meaning of
/// a security descriptor or its components. Each security descriptor has a `control` member that
/// stores the [`SECURITY_DESCRIPTOR_CONTROL`] bits.
#[allow(non_camel_case_types)]
pub type SECURITY_DESCRIPTOR_CONTROL = WORD;

/// A [`WCHAR`]
pub type TCHAR = WCHAR;

/// A 16-bit Unicode character
pub type WCHAR = u16;
