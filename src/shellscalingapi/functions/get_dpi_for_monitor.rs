use crate::{HMONITOR, HRESULT, MONITOR_DPI_TYPE, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{E_INVALIDARG, S_OK};

#[link(name = "Shcore")]
unsafe extern "system" {
    /// Queries the dots per inch (dpi) of a display.
    ///
    /// # Parameters
    ///  * `monitor` - Handle of the monitor being queried.
    ///  * `dpi_type` - The type of DPI being queried. Possible values are from the
    ///                 [`MONITOR_DPI_TYPE`] enumeration.
    ///  * `dpi_x` - The value of the DPI along the X axis. This value always refers to the
    ///              horizontal edge, even when the screen is rotated.
    ///  * `dpi_y` - The value of the DPI along the Y axis. This value always refers to the
    ///              vertical edge, even when the screen is rotated.
    ///
    /// # Return Value
    /// This function returns one of the following values:
    ///  * [`S_OK`] -  The function successfully returns the X and Y DPI values for the specified
    ///                monitor.
    ///  * [`E_INVALIDARG`] -  The handle, DPI type, or pointers passed in are not valid.
    ///
    /// # Remarks
    /// This API is not DPI aware and should not be used if the calling thread is per-monitor DPI
    /// aware. For the DPI-aware version of this API, see [`GetDpiForWindow`].
    ///
    /// When you call [`GetDpiForMonitor`], you will receive different DPI values depending on the
    /// DPI awareness of the calling application. DPI awareness is an application-level property
    /// usually defined in the application manifest. For more information about DPI awareness
    /// values, see [`PROCESS_DPI_AWARENESS`]. The following indicates how the results will differ
    /// based on the [`PROCESS_DPI_AWARENESS`] value of your application:
    ///  * [`PROCESS_DPI_UNAWARE`] - 96 because the app is unaware of any other scale factors.
    ///  * [`PROCESS_SYSTEM_DPI_AWARE`] - A value set to the system DPI because the app assumes all
    ///                                   applications use the system DPI.
    ///  * [`PROCESS_PER_MONITOR_DPI_AWARE`] - The actual DPI value set by the user for that
    ///                                        display.
    ///
    /// The values of `*dpi_x` and `*dpi_y` are identical. You only need to record one of the
    /// values to determine the DPI and respond appropriately.
    ///
    /// When [`MONITOR_DPI_TYPE`] is [`MONITOR_DPI_TYPE::AngularDpi`] or
    /// [`MONITOR_DPI_TYPE::RawDpi`], the returned DPI value does not include any changes that the
    /// user made to the DPI by using the desktop scaling override slider control in Control Panel.
    pub fn GetDpiForMonitor(
        monitor: HMONITOR,
        dpi_type: MONITOR_DPI_TYPE,
        dpi_x: *mut UINT,
        dpi_y: *mut UINT,
    ) -> HRESULT;
}
