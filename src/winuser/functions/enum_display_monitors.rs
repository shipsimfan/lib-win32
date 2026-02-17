use crate::{BOOL, HDC, LPARAM, LPCRECT, MONITORENUMPROC};

// rustdoc imports
#[allow(unused_imports)]
use crate::RECT;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// The [`EnumDisplayMonitors`] function enumerates display monitors (including invisible
    /// pseudo-monitors associated with the mirroring drivers) that intersect a region formed by
    /// the intersection of a specified clipping rectangle and the visible region of a device
    /// context. [`EnumDisplayMonitors`] calls an application-defined [`MONITORENUMPROC`] callback
    /// function once for each monitor that is enumerated. Note that [`GetSystemMetrics`]
    /// ([`SM_CMONITORS`]) counts only the display monitors.
    ///
    /// # Parameters
    ///  * `hdc` - A handle to a display device context that defines the visible region of
    ///            interest. If this parameter is [`null_mut`], the `hdc_monitor` parameter passed
    ///            to the callback function will be [`null_mut`], and the visible region of
    ///            interest is the virtual screen that encompasses all the displays on the desktop.
    ///  * `clip` - A pointer to a [`RECT`] structure that specifies a clipping rectangle. The
    ///             region of interest is the intersection of the clipping rectangle with the
    ///             visible region specified by hdc. If `hdc` is not [`null_mut`], the coordinates
    ///             of the clipping rectangle are relative to the origin of the `hdc`. If `hdc` is
    ///             [`null_mut`], the coordinates are virtual-screen coordinates. This parameter
    ///             can be [`null_mut`] if you don't want to clip the region specified by `hdc`.
    ///  * `r#enum` - A pointer to a [`MONITORENUMPROC`] application-defined callback function.
    ///  * `data` - Application-defined data that [`EnumDisplayMonitors`] passes directly to the
    ///             [`MONITORENUMPROC`] function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero.
    ///
    /// # Remarks
    /// There are two reasons to call the [`EnumDisplayMonitors`] function:
    ///  - You want to draw optimally into a device context that spans several display monitors,
    ///    and the monitors have different color formats.
    ///  - You want to obtain a handle and position rectangle for one or more display monitors.
    ///
    /// To determine whether all the display monitors in a system share the same color format, call
    /// [`GetSystemMetrics`] ([`SM_SAMEDISPLAYFORMAT`]).
    ///
    /// You do not need to use the [`EnumDisplayMonitors`] function when a window spans display
    /// monitors that have different color formats. You can continue to paint under the assumption
    /// that the entire screen has the color properties of the primary monitor. Your windows will
    /// look fine. [`EnumDisplayMonitors`] just lets you make them look better.
    ///
    /// Setting the `hdc` parameter to [`null_mut`] lets you use the [`EnumDisplayMonitors`]
    /// function to obtain a handle and position rectangle for one or more display monitors.
    pub fn EnumDisplayMonitors(
        hdc: HDC,
        clip: LPCRECT,
        r#enum: MONITORENUMPROC,
        data: LPARAM,
    ) -> BOOL;
}
