use crate::{DWORD, HANDLE};
use std::ffi::{c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, LONG, TRUE};

/// A Boolean variable (should be [`TRUE`] or [`FALSE`])
pub type BOOL = c_int;

/// A byte (8 bits)
pub type BYTE = c_uchar;

/// A handle to a global memory block
pub type HGLOBAL = HANDLE;

/// A handle to a registry key
pub type HKEY = HANDLE;

/// A handle to a local memory block
pub type HLOCAL = HANDLE;

/// A 32-bit signed integer. The range is -2147483648 through 2147483647 decimal.
pub type INT = c_int;

/// A pointer to a constant of any type
pub type LPCVOID = *const c_void;

/// A pointer to a [`DWORD`]
pub type LPDWORD = *mut DWORD;

/// A pointer to any type
pub type LPVOID = *mut c_void;

/// A pointer to a [`DWORD`]
pub type PDWORD = *mut DWORD;

/// A pointer to an [`HKEY`]
pub type PHKEY = *mut HKEY;

/// An unsigned [`INT`]. The range is 0 through 4294967295 decimal.
pub type UINT = c_uint;

/// An unsigned [`LONG`]. The range is 0 through 4294967295 decimal.
pub type ULONG = c_ulong;

/// A 16-bit unsigned integer. The range is 0 through 65535 decimal.
pub type WORD = c_ushort;
