use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{HKEY_LOCAL_MACHINE, HKEY_USERS};

/// The key or hive is saved in standard format. The standard format is the only format supported
/// by Windows 2000.
pub const REG_STANDARD_FORMAT: DWORD = 1;

/// The key or hive is saved in the latest format. The latest format is supported starting with
/// Windows XP. After the key or hive is saved in this format, it cannot be loaded on an earlier
/// system.
pub const REG_LATEST_FORMAT: DWORD = 2;

/// The hive is saved with no compression, for faster save operations. The `key` parameter must
/// specify the root of a hive under [`HKEY_LOCAL_MACHINE`] or [`HKEY_USERS`]. For example,
/// `HKLM\SOFTWARE` is the root of a hive.
pub const REG_NO_COMPRESSION: DWORD = 4;
