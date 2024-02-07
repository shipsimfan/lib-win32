/// An unsigned long type for pointer precision. Use when casting a pointer to a long type to
/// perform pointer arithmetic. (Also commonly used for general 32-bit parameters that have been
/// extended to 64 bits in 64-bit Windows.)
#[allow(non_camel_case_types)]
pub type DWORD_PTR = ULONG_PTR;

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

/// An unsigned [`LONG_PTR`]
#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types)]
pub type ULONG_PTR = u64;

/// An unsigned [`LONG_PTR`]
#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types)]
pub type ULONG_PTR = std::ffi::c_ulong;
