use core::ffi::*;

/// An 8-bit Windows (ANSI) character.
pub type Char = c_char;

/// A 32-bit unsigned integer. The range is 0 through 4294967295 decimal.
pub type DWord = c_ulong;

/// An unsigned long type for pointer precision. Use when casting a pointer to
/// a long type to perform pointer arithmetic. (Also commonly used for general
/// 32-bit parameters that have been extended to 64 bits in 64-bit Windows.)
pub type DWordPtr = ULongPtr;

/// A 32-bit signed integer. The range is -2147483648 through 2147483647
/// decimal.
pub type Int = c_int;

/// A signed integer type for pointer precision. Use when casting a pointer to
/// an integer to perform pointer arithmetic.
pub type IntPtr = i64;

/// A 32-bit signed integer. The range is -2147483648 through 2147483647
/// decimal.
pub type Long = c_long;

/// A signed long type for pointer precision. Use when casting a pointer to a
/// long to perform pointer arithmetic.
pub type LongPtr = i64;

/// The maximum number of bytes to which a pointer can point. Use for a count
/// that must span the full range of a pointer.
pub type SizeT = ULongPtr;

/// An unsigned [`Int`]. The range is 0 through 4294967295 decimal.
pub type UInt = c_uint;

/// An unsigned [`IntPtr`].
pub type UIntPtr = u64;

/// An unsigned [`LongPtr`].
pub type ULongPtr = u64;

/// Any type.
pub type Void = c_void;

/// A 16-bit Unicode character.
pub type WChar = u16;

/// A 16-bit unsigned integer. The range is 0 through 65535 decimal.
pub type Word = c_ushort;
