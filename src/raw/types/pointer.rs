use crate::raw::{Char, WChar};
use std::ffi::c_void;

/// A pointer to a constant of any type.
pub type LPCVoid = *const c_void;

/// A pointer to a null-terminated string of 16-bit Unicode characters.
pub type LPWStr = *mut WChar;

/// A pointer to a constant null-terminated string of 8-bit Windows (ANSI)
/// characters.
pub type PCStr = *const Char;

/// A pointer to any type.
pub type PVoid = *mut c_void;

/// A pointer to a null-terminated string of 16-bit Unicode characters.
pub type PWStr = *mut WChar;
