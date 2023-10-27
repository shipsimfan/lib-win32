use super::WChar;
use std::ffi::c_void;

/// A pointer to a constant of any type.
pub type LPCVoid = *const c_void;

/// A pointer to a null-terminated string of 16-bit Unicode characters.
pub type LPWStr = *mut WChar;
