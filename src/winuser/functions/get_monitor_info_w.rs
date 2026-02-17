use crate::{BOOL, HMONITOR, LPMONITORINFO};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetMonitorInfo, MONITORINFO, MONITORINFOEX};

#[link(name = "User32")]
unsafe extern "system" {
    /// The [`GetMonitorInfo`] function retrieves information about a display monitor.
    ///
    /// # Parameters
    ///  * `monitor` - A handle to the display monitor of interest.
    ///  * `mi` - A pointer to a [`MONITORINFO`] or [`MONITORINFOEX`] structure that receives
    ///           information about the specified display monitor. You must set the `size` member
    ///           of the structure to `std::mem::size_of::<MONITORINFO>()` or
    ///           `std::mem::size_of::<MONITORINFOEX>()` before calling the [`GetMonitorInfo`]
    ///           function. Doing so lets the function determine the type of structure you are
    ///           passing to it. The [`MONITORINFOEX`] structure is a superset of the
    ///           [`MONITORINFO`] structure. It has one additional member: a string that contains a
    ///           name for the display monitor. Most applications have no use for a display monitor
    ///           name, and so can save some bytes by using a [`MONITORINFO`] structure.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero.
    pub fn GetMonitorInfoW(monitor: HMONITOR, mi: LPMONITORINFO) -> BOOL;
}
