use crate::DWORD;

/// Restore whole hive volatile
pub const REG_WHOLE_HIVE_VOLATILE: DWORD = 0x00000001;

/// Unwind changes to last flush
pub const REG_REFRESH_HIVE: DWORD = 0x00000002;

/// Never lazy flush this hive
pub const REG_NO_LAZY_FLUSH: DWORD = 0x00000004;

/// Force the restore process even when we have open handles on subkeys
pub const REG_FORCE_RESTORE: DWORD = 0x00000008;

/// Loads the hive visible to the calling process
pub const REG_APP_HIVE: DWORD = 0x00000010;

/// Hive cannot be mounted by any other process while in use
pub const REG_PROCESS_PRIVATE: DWORD = 0x00000020;

/// Starts Hive Journal
pub const REG_START_JOURNAL: DWORD = 0x00000040;

/// Grow hive file in exact 4k increments
pub const REG_HIVE_EXACT_FILE_GROWTH: DWORD = 0x00000080;

/// No RM is started for this hive (no transactions)
pub const REG_HIVE_NO_RM: DWORD = 0x00000100;

/// Legacy single logging is used for this hive
pub const REG_HIVE_SINGLE_LOG: DWORD = 0x00000200;

/// This hive might be used by the OS loader
pub const REG_BOOT_HIVE: DWORD = 0x00000400;

/// Load the hive and return a handle to its root kcb
pub const REG_LOAD_HIVE_OPEN_HANDLE: DWORD = 0x00000800;

/// Flush changes to primary hive file size as part of all flushes
pub const REG_FLUSH_HIVE_FILE_GROWTH: DWORD = 0x00001000;

/// Open a hive's files in read-only mode
pub const REG_OPEN_READ_ONLY: DWORD = 0x00002000;

/// Load the hive, but don't allow any modification of it
pub const REG_IMMUTABLE: DWORD = 0x00004000;

/// Do not fall back to impersonating the caller if hive file access fails
pub const REG_NO_IMPERSONATION_FALLBACK: DWORD = 0x00008000;

/// Open an app hive's files in read-only mode (if the hive was not previously loaded)
pub const REG_APP_HIVE_OPEN_READ_ONLY: DWORD = REG_OPEN_READ_ONLY;
