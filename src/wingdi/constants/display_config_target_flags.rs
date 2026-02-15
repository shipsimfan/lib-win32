use crate::UINT32;

// rustdoc imports
#[allow(unused_imports)]
use crate::{QueryDisplayConfig, QDC_INCLUDE_HMD};

/// Target is in use on an active path.
pub const DISPLAYCONFIG_TARGET_IN_USE: UINT32 = 0x00000001;

/// The output can be forced on this target even if a monitor is not detected.
pub const DISPLAYCONFIG_TARGET_FORCIBLE: UINT32 = 0x00000002;

/// Output is currently being forced in a boot-persistent manner.
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_BOOT: UINT32 = 0x00000004;

/// Output is currently being forced in a path-persistent manner.
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_PATH: UINT32 = 0x00000008;

/// Output is currently being forced in a nonpersistent manner.
pub const DISPLAYCONFIG_TARGET_FORCED_AVAILABILITY_SYSTEM: UINT32 = 0x00000010;

/// The output is a head-mounted display (HMD). Such a path is only returned from
/// [`QueryDisplayConfig`] using the [`QDC_INCLUDE_HMD`] flag.
///
/// Supported starting in the Windows 10 Creators Update (Version 1703).
pub const DISPLAYCONFIG_TARGET_IS_HMD: UINT32 = 0x00000020;
