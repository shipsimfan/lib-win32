use crate::{CCHDEVICENAME, CHAR, MONITORINFO};

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
pub struct MONITORINFOEXA {
    /// The [`MONITORINFOEXA`] structure implements [`MONITORINFO`].
    pub info: MONITORINFO,

    /// A string that specifies the device name of the monitor being used. Most applications have
    /// no use for a display monitor name, and so can save some bytes by using a [`MONITORINFO`]
    /// structure.
    pub device: [CHAR; CCHDEVICENAME],
}

impl Default for MONITORINFOEXA {
    fn default() -> Self {
        MONITORINFOEXA {
            info: MONITORINFO {
                size: std::mem::size_of::<MONITORINFOEXA>() as _,
                ..Default::default()
            },
            device: [0; CCHDEVICENAME],
        }
    }
}
