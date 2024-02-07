use std::ffi::{c_ushort, c_void};

/// A pointer to a constant of any type
pub type LPCVOID = *const c_void;

/// A 16-bit unsigned integer. The range is 0 through 65535 decimal.
pub type WORD = c_ushort;
