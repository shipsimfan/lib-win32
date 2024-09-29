use crate::{DWORD, HANDLE, LONG_PTR, RECT, UINT_PTR};
use std::ffi::{c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, LONG, TRUE};

#[allow(missing_docs)]
pub type ATOM = WORD;

/// A Boolean variable (should be [`TRUE`] or [`FALSE`])
pub type BOOL = c_int;

/// A byte (8 bits)
pub type BYTE = c_uchar;

/// A handle to a brush.
pub type HBRUSH = HANDLE;

/// A handle to a cursor.
pub type HCURSOR = HANDLE;

/// A handle to a device context (DC).
pub type HDC = HANDLE;

/// A handle to a global memory block
pub type HGLOBAL = HANDLE;

/// A handle to an icon
pub type HICON = HANDLE;

/// A handle to an instance. This is the base address of the module in memory.
pub type HINSTANCE = HANDLE;

/// A handle to a registry key
pub type HKEY = HANDLE;

/// A handle to a local memory block
pub type HLOCAL = HANDLE;

/// A handle to a menu.
pub type HMENU = HANDLE;

/// A handle to a module. This is the base address of the module in memory.
pub type HMODULE = HANDLE;

/// A handle to a window.
pub type HWND = HANDLE;

/// A 32-bit signed integer. The range is -2147483648 through 2147483647 decimal.
pub type INT = c_int;

/// A message parameter.
pub type LPARAM = LONG_PTR;

/// A pointer to a [`BYTE`]
pub type LPBYTE = *mut BYTE;

/// A pointer to a constant of any type
pub type LPCVOID = *const c_void;

/// A pointer to a [`DWORD`]
pub type LPDWORD = *mut DWORD;

/// A pointer to an [`INT`]
pub type LPINT = *mut c_int;

/// A pointer to a [`RECT`]
pub type LPRECT = *mut RECT;

/// A pointer to any type
pub type LPVOID = *mut c_void;

/// Signed result of message processing
pub type LRESULT = LONG_PTR;

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

/// A message parameter.
pub type WPARAM = UINT_PTR;
