use crate::DWORD64;

/// The allocation is mapped using large pages.
pub const MEM_EXTENDED_PARAMETER_NONPAGED_LARGE: DWORD64 = 0x08;

/// The allocation is mapped using huge pages.
pub const MEM_EXTENDED_PARAMETER_NONPAGED_HUGE: DWORD64 = 0x10;

/// The allocation will contain emulation-compatible (EC) code.
pub const MEM_EXTENDED_PARAMETER_EC_CODE: DWORD64 = 0x40;
