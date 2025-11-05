use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::RAWINPUT;

/// Get the header information from the [`RAWINPUT`] structure.
pub const RID_HEADER: UINT = 0x10000005;

/// Get the raw data from the [`RAWINPUT`] structure.
pub const RID_INPUT: UINT = 0x10000003;
