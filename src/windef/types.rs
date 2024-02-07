use crate::HANDLE;
use std::ffi::{c_uint, c_ushort, c_void};

/// A handle to a local memory block
pub type HLOCAL = HANDLE;

/// A pointer to a constant of any type
pub type LPCVOID = *const c_void;

/// An unsigned INT. The range is 0 through 4294967295 decimal.
pub type UINT = c_uint;

/// A 16-bit unsigned integer. The range is 0 through 65535 decimal.
pub type WORD = c_ushort;