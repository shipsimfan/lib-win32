use crate::DWORD;

/// Retrieve the current settings for the display device.
pub const ENUM_CURRENT_SETTINGS: DWORD = -1i32 as DWORD;

/// Retrieve the settings for the display device that are currently stored in the registry.
pub const ENUM_REGISTRY_SETTINGS: DWORD = -2i32 as DWORD;
