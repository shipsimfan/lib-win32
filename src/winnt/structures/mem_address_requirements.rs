use crate::{PVOID, SIZE_T};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Specifies a lowest and highest base address and alignment as part of an extended parameter to a
/// function that manages virtual memory.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct MEM_ADDRESS_REQUIREMENTS {
    /// Specifies the lowest acceptable address. This address must be a multiple of the allocation
    /// granularity returned by [`GetSystemInfo`], or a multiple of the large page size returned by
    /// [`GetLargePageMinimum`] if large pages are being requested. If this member is [`null_mut`],
    /// then there is no lower limit.
    pub lowest_starting_address: PVOID,

    /// Specifies the highest acceptable address (inclusive). This address must not exceed
    /// `maximum_application_address` returned by [`GetSystemInfo`]. If this member is
    /// [`null_mut`], then there is no upper limit.
    pub highest_ending_address: PVOID,

    /// Specifies power-of-2 alignment. Specifying 0 aligns the returned address on the system
    /// allocation granularity.
    pub alignment: SIZE_T,
}
