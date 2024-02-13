use crate::DWORD;

/// No value type
pub const REG_NONE: DWORD = 0;

/// Unicode nul terminated string
pub const REG_SZ: DWORD = 1;

/// Unicode nul terminated string (with environment variable references)
pub const REG_EXPAND_SZ: DWORD = 2;

/// Free form binary
pub const REG_BINARY: DWORD = 3;

/// 32-bit number
pub const REG_DWORD: DWORD = 4;

/// 32-bit number (same as [`REG_DWORD`])
pub const REG_DWORD_LITTLE_ENDIAN: DWORD = 4;

/// 32-bit number
pub const REG_DWORD_BIG_ENDIAN: DWORD = 5;

/// Symbolic Link (unicode)
pub const REG_LINK: DWORD = 6;

/// Multiple Unicode strings
pub const REG_MULTI_SZ: DWORD = 7;

/// Resource list in the resource map
pub const REG_RESOURCE_LIST: DWORD = 8;

/// Resource list in the hardware description
pub const REG_FULL_RESOURCE_DESCRIPTOR: DWORD = 9;

/// 64-bit number
pub const REG_QWORD: DWORD = 11;

/// 64-bit number (same as [`REG_QWORD`])
pub const REG_QWORD_LITTLE_ENDIAN: DWORD = 11;
