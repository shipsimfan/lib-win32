use crate::{CCHDEVICENAME, MONITORINFO, WCHAR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetMonitorInfo, MONITORINFOEX};

/// The [`MONITORINFOEX`] structure contains information about a display monitor.
///
/// The [`GetMonitorInfo`] function stores information into a [`MONITORINFOEX`] structure or a
/// [`MONITORINFO`] structure.
///
/// The [`MONITORINFOEX`] structure is a superset of the [`MONITORINFO`] structure. The
/// [`MONITORINFOEX`] structure adds a string member to contain a name for the display monitor.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MONITORINFOEXW {
    /// The [`MONITORINFOEXW`] structure implements [`MONITORINFO`].
    pub info: MONITORINFO,

    /// A string that specifies the device name of the monitor being used. Most applications have
    /// no use for a display monitor name, and so can save some bytes by using a [`MONITORINFO`]
    /// structure.
    pub device: [WCHAR; CCHDEVICENAME],
}

impl Default for MONITORINFOEXW {
    fn default() -> Self {
        MONITORINFOEXW {
            info: MONITORINFO {
                size: std::mem::size_of::<MONITORINFOEXW>() as _,
                ..Default::default()
            },
            device: [0; CCHDEVICENAME],
        }
    }
}
