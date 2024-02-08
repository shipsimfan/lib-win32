use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

/// Treat [`null`] string pointers as empty strings. Don't fault on [`null`] buffers.
pub const STRSAFE_IGNORE_NULLS: DWORD = 0x00000100;

/// On success, fill in extra space behind the null terminator with fill pattern
pub const STRSAFE_FILL_BEHIND_NULL: DWORD = 0x00000200;

/// On failure, overwrite `psz_dest` with fill pattern and null terminate it
pub const STRSAFE_FILL_ON_FAILURE: DWORD = 0x00000400;

/// On failure, set `pst_dest` to [`null`]
pub const STRSAFE_NULL_ON_FAILURE: DWORD = 0x00000800;

/// Instead of returning a truncated result, copy/append nothing to `psz_dest` and null terminate
/// it
pub const STRSAFE_NO_TRUNCATION: DWORD = 0x00001000;
