use crate::DWORD;

/// Read access
pub const GENERIC_READ: DWORD = 0x80000000;

/// Write access
pub const GENERIC_WRITE: DWORD = 0x40000000;

/// Execute access
pub const GENERIC_EXECUTE: DWORD = 0x20000000;

/// Read, write, and execute access
pub const GENERIC_ALL: DWORD = 0x10000000;
