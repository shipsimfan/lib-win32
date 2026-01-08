use std::ffi::{c_uchar, c_ushort};

/// A 64-bit unsigned integer.
pub type DWORD64 = u64;

/// An unsigned long type for pointer precision. Use when casting a pointer to a long type to
/// perform pointer arithmetic. (Also commonly used for general 32-bit parameters that have been
/// extended to 64 bits in 64-bit Windows.)
#[allow(non_camel_case_types)]
pub type DWORD_PTR = ULONG_PTR;

/// A signed integer type for pointer precision. Use when casting a pointer to an integer to
/// perform pointer arithmetic.
#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
pub type INT_PTR = i64;

/// A signed integer type for pointer precision. Use when casting a pointer to an integer to
/// perform pointer arithmetic.
#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
pub type INT_PTR = std::ffi::c_int;

/// A 64-bit signed integer. The range is -9223372036854775808 through 9223372036854775807 decimal.
pub type LONG64 = i64;

/// A signed long type for pointer precision. Use when casting a pointer to a long to perform
/// pointer arithmetic.
#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
pub type LONG_PTR = i64;

/// A signed long type for pointer precision. Use when casting a pointer to a long to perform
/// pointer arithmetic.
#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
pub type LONG_PTR = std::ffi::c_long;

/// A pointer to a [`DWORD_PTR`]
#[allow(non_camel_case_types)]
pub type PDWORD_PTR = *mut DWORD_PTR;

/// A pointer to a [`ULONG_PTR`]
#[allow(non_camel_case_types)]
pub type PULONG_PTR = *mut ULONG_PTR;

/// The maximum number of bytes to which a pointer can point. Use for a count that must span the
/// full range of a pointer.
#[allow(non_camel_case_types)]
pub type SIZE_T = ULONG_PTR;

/// An unsigned [`INT8`]
pub type UINT8 = c_uchar;

/// An unsigned [`INT16`]
pub type UINT16 = c_ushort;

/// An unsigned [`INT64`]. The range is 0 through 18446744073709551615 decimal.
pub type UINT64 = u64;

/// An unsigned [`INT_PTR`]
#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
pub type UINT_PTR = u64;

/// An unsigned [`INT_PTR`]
#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
pub type UINT_PTR = std::ffi::c_uint;

/// An unsigned [`LONG64`]. The range is 0 through 18446744073709551615 decimal.
pub type ULONG64 = u64;

/// An unsigned [`LONG_PTR`]
#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
pub type ULONG_PTR = u64;

/// An unsigned [`LONG_PTR`]
#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
pub type ULONG_PTR = std::ffi::c_ulong;
