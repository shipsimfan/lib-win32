use std::ffi::c_short;

/// Print single-sided.
pub const DMDUP_SIMPLEX: c_short = 1;

/// Print double-sided, using long edge binding.
pub const DMDUP_VERTICAL: c_short = 2;

/// Print double-sided, using short edge binding.
pub const DMDUP_HORIZONTAL: c_short = 3;
