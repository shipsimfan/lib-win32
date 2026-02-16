use std::ffi::c_short;

/// Do not collate when printing multiple copies.
pub const DMCOLLATE_FALSE: c_short = 0;

/// Collate when printing multiple copies.
pub const DMCOLLATE_TRUE: c_short = 1;
