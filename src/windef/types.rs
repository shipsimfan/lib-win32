use crate::HANDLE;
use std::ffi::{c_int, c_uint, c_ushort, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, TRUE};

/// A Boolean variable (should be [`TRUE`] or [`FALSE`]).
pub type BOOL = c_int;

/// A handle to a global memory block
pub type HGLOBAL = HANDLE;

/// A handle to a local memory block
pub type HLOCAL = HANDLE;

/// A pointer to a constant of any type
pub type LPCVOID = *const c_void;

/// A pointer to any type
pub type LPVOID = *mut c_void;

/// An unsigned INT. The range is 0 through 4294967295 decimal.
pub type UINT = c_uint;

/// A 16-bit unsigned integer. The range is 0 through 65535 decimal.
pub type WORD = c_ushort;
