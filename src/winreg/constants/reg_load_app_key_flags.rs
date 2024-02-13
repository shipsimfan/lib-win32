use crate::DWORD;

/// This prevents access to this registry hive by another caller.
pub const REG_PROCESS_APPKEY: DWORD = 0x00000001;
