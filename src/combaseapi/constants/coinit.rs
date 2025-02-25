use crate::DWORD;

/// Initializes the thread for apartment-threaded object concurrency.
pub const COINIT_APARTMENTTHREADED: DWORD = 0x2;

/// Initializes the thread for multithreaded object concurrency.
pub const COINIT_MULTITHREADED: DWORD = 0x0;

/// Disables DDE for OLE1 support.
pub const COINIT_DISABLE_OLE1DDE: DWORD = 0x4;

/// Increase memory usage in an attempt to increase performance.
pub const COINIT_SPEED_OVER_MEMORY: DWORD = 0x8;
