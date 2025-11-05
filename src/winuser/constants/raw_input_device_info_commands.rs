use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateFile, ReadFile, WriteFile, PHIDP_PREPARSED_DATA, RID_DEVICE_INFO};

/// `data` is a [`PHIDP_PREPARSED_DATA`] pointer to a buffer for a top-level collection's preparsed
/// data.
pub const RIDI_PREPARSEDDATA: UINT = 0x20000005;

/// `data` points to a string that contains the device interface name.
///
/// If this device is opened with Shared Access Mode then you can call [`CreateFile`] with this
/// name to open a HID collection and use returned handle for calling [`ReadFile`] to read input
/// reports and [`WriteFile`] to send output reports.
///
/// For this `command` only, the value in `size` is the character count (not the byte count).
pub const RIDI_DEVICENAME: UINT = 0x20000007;

/// `data` points to an [`RID_DEVICE_INFO`] structure.
pub const RIDI_DEVICEINFO: UINT = 0x2000000B;
