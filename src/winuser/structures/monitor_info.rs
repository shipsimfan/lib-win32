use crate::{DWORD, RECT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetMonitorInfo, MONITORINFOEX, MONITORINFOF_PRIMARY};

/// The [`MONITORINFO`] structure contains information about a display monitor.
///
/// The [`GetMonitorInfo`] function stores information in a [`MONITORINFO`] structure or a
/// [`MONITORINFOEX`] structure.
///
/// The [`MONITORINFO`] structure is a subset of the [`MONITORINFOEX`] structure. The
/// [`MONITORINFOEX`] structure adds a string member to contain a name for the display monitor.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MONITORINFO {
    /// The size of the structure, in bytes.
    ///
    /// Set this member to `std::mem::size_of::<MONITORINFO>()` before calling the
    /// [`GetMonitorInfo`] function. Doing so lets the function determine the type of structure you
    /// are passing to it.
    pub size: DWORD,

    /// A [`RECT`] structure that specifies the display monitor rectangle, expressed in
    /// virtual-screen coordinates. Note that if the monitor is not the primary display monitor,
    /// some of the rectangle's coordinates may be negative values.
    pub monitor: RECT,

    /// A [`RECT`] structure that specifies the work area rectangle of the display monitor,
    /// expressed in virtual-screen coordinates. Note that if the monitor is not the primary
    /// display monitor, some of the rectangle's coordinates may be negative values.
    pub work: RECT,

    /// A set of flags that represent attributes of the display monitor.
    ///
    /// The following flag is defined:
    ///  * [`MONITORINFOF_PRIMARY`] - This is the primary display monitor.
    pub flags: DWORD,
}

impl Default for MONITORINFO {
    fn default() -> Self {
        MONITORINFO {
            size: std::mem::size_of::<MONITORINFO>() as _,
            monitor: RECT::default(),
            work: RECT::default(),
            flags: 0,
        }
    }
}
