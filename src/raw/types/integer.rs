use core::ffi::*;

/// A 32-bit unsigned integer. The range is 0 through 4294967295 decimal.
pub type DWord = c_ulong;

/// An unsigned long type for pointer precision. Use when casting a pointer to
/// a long type to perform pointer arithmetic. (Also commonly used for general
/// 32-bit parameters that have been extended to 64 bits in 64-bit Windows.)
pub type DWordPtr = ULongPtr;

/// A signed long type for pointer precision. Use when casting a pointer to a
/// long to perform pointer arithmetic.
pub type LongPtr = i64;

/// An unsigned [`LongPtr`].
pub type ULongPtr = u64;

/// A 16-bit Unicode character. For more information, see Character Sets Used By Fonts.
pub type WChar = u16;

/// A 16-bit unsigned integer. The range is 0 through 65535 decimal.
pub type Word = c_ushort;
