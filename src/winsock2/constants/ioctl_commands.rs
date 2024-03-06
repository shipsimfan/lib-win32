use std::ffi::c_long;

/// Set or clear non-blocking I/O
pub const FIONBIO: c_long = 0x8004667Eu32 as _;
