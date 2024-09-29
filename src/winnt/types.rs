use crate::{ACL, DWORD, SECURITY_DESCRIPTOR, SID, WORD};
use std::ffi::{c_char, c_long, c_short, c_void};

/// The [`ACCESS_MASK`] data type is a [`DWORD`] value that defines standard, specific, and generic
/// rights. These rights are used in access control entries (ACEs) and are the primary means of
/// specifying the requested or granted access to an object.
#[allow(non_camel_case_types)]
pub type ACCESS_MASK = DWORD;

/// 	An 8-bit Windows (ANSI) character.
pub type CHAR = c_char;

/// A handle to an object
pub type HANDLE = PVOID;

/// The return codes used by COM interfaces
pub type HRESULT = LONG;

/// A language identifier
pub type LANGID = WORD;

/// A 64-bit signed integer. The range is -9223372036854775808 through 9223372036854775807 decimal.
pub type LONG = c_long;

/// A 64-bit signed integer. The range is -9223372036854775808 through 9223372036854775807 decimal.
pub type LONGLONG = i64;

/// A pointer to a constant null-terminated string of 8-bit Windows (ANSI) characters.
pub type LPCSTR = *const CHAR;

/// A pointer to a constant null-terminated string of 16-bit Unicode characters
pub type LPCWSTR = *const WCHAR;

/// An [`LPWSTR`]
pub type LPTSTR = LPWSTR;

/// A pointer to a null-terminated string of 16-bit Unicode characters
pub type LPWSTR = *mut WCHAR;

/// A pointer to an [`ACL`]
pub type PACL = *mut ACL;

/// A pointer to a [`SECURITY_DESCRIPTOR`] structure
#[allow(non_camel_case_types)]
pub type PSECURITY_DESCRIPTOR = *mut SECURITY_DESCRIPTOR;

/// A pointer to a [`SID`] structure
pub type PSID = *mut SID;

/// A pointer to any type
pub type PVOID = *mut c_void;

/// The [`SECURITY_DESCRIPTOR_CONTROL`] data type is a set of bit flags that qualify the meaning of
/// a security descriptor or its components. Each security descriptor has a `control` member that
/// stores the [`SECURITY_DESCRIPTOR_CONTROL`] bits.
#[allow(non_camel_case_types)]
pub type SECURITY_DESCRIPTOR_CONTROL = WORD;

/// The [`SECURITY_INFORMATION`] data type identifies the object-related security information being
/// set or queried. This security information includes:
///  - The owner of an object
///  - The primary group of an object
///  - The discretionary access control list (DACL) of an object
///  - The system access control list (SACL) of an object
#[allow(non_camel_case_types)]
pub type SECURITY_INFORMATION = DWORD;

/// A 16-bit integer. The range is -32768 through 32767 decimal.
pub type SHORT = c_short;

/// A [`WCHAR`]
pub type TCHAR = WCHAR;

/// A 16-bit Unicode character
pub type WCHAR = u16;
