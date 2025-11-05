use std::ffi::c_ulong;

/// A 32-bit unsigned integer. The range is 0 through 4294967295 decimal.
pub type DWORD = c_ulong;

/// A 64-bit unsigned integer.
pub type QWORD = u64;
