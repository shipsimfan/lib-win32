use crate::DWORD;

/// Parameter is reserved
pub const REG_OPTION_RESERVED: DWORD = 0x00000000;

/// Key is preserved when system is rebooted
pub const REG_OPTION_NON_VOLATILE: DWORD = 0x00000000;

/// Key is not preserved when system is rebooted
pub const REG_OPTION_VOLATILE: DWORD = 0x00000001;

/// Created key is a symbolic link
pub const REG_OPTION_CREATE_LINK: DWORD = 0x00000002;

/// open for backup or restore special access rules privilege required
pub const REG_OPTION_BACKUP_RESTORE: DWORD = 0x00000004;

/// Open symbolic link
pub const REG_OPTION_OPEN_LINK: DWORD = 0x00000008;

/// Disable Open/Read/Write virtualization for this open and the resulting handle.
pub const REG_OPTION_DONT_VIRTUALIZE: DWORD = 0x00000010;
