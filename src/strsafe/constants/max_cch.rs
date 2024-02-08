use core::ffi::c_size_t;

// rustdoc imports
#[allow(unused_imports)]
use std::ffi::c_int;

/// Max buffer size, in characters, that we support (same as [`c_int::MAX`])
pub const STRSAFE_MAX_CCH: c_size_t = 2147483647;
