use crate::UINT32;

/// Returns all the possible path combinations of sources to targets.
pub const QDC_ALL_PATHS: UINT32 = 0x00000001;

/// Returns currently active paths only.
pub const QDC_ONLY_ACTIVE_PATHS: UINT32 = 0x00000002;

/// Returns active paths as defined in the CCD database for the currently connected displays.
pub const QDC_DATABASE_CURRENT: UINT32 = 0x00000004;

/// This flag should be bitwise OR'ed with other flags to indicate that the caller is aware of
/// virtual mode support.
///
/// Supported starting in Windows 10.
pub const QDC_VIRTUAL_MODE_AWARE: UINT32 = 0x00000010;

/// This flag should be bitwise OR'ed with QDC_ONLY_ACTIVE_PATHS to indicate that the caller would
/// like to include head-mounted displays (HMDs) in the list of active paths.
///
/// Supported starting in Windows 10 1703 Creators Update.
pub const QDC_INCLUDE_HMD: UINT32 = 0x00000020;

/// This flag should be bitwise OR'ed with other flags to indicate that the caller is aware of
/// virtual refresh rate support.
///
/// Supported starting in Windows 11.
pub const QDC_VIRTUAL_REFRESH_RATE_AWARE: UINT32 = 0x00000040;
