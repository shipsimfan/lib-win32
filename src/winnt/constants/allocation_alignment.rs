// rustdoc imports
#[allow(unused_imports)]
use crate::HeapAlloc;

/// The alignment of memory returned by [`HeapAlloc`]
#[cfg(target_pointer_width = "64")]
pub const MEMORY_ALLOCATION_ALIGNMENT: usize = 16;

/// The alignment of memory returned by [`HeapAlloc`]
#[cfg(target_pointer_width = "32")]
pub const MEMORY_ALLOCATION_ALIGNMENT: usize = 8;
