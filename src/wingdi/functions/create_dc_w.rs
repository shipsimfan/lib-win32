use crate::{DEVMODEW, HDC, LPCWSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateDC, EnumDisplayDevices, DEVMODE};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Gdi32")]
unsafe extern "system" {
    /// The [`CreateDC`] function creates a device context (DC) for a device using the specified
    /// name.
    ///
    /// # Parameters
    ///  * `driver` - A pointer to a null-terminated character string that specifies either
    ///               `DISPLAY` or the name of a specific display device. For printing, we
    ///               recommend that you pass [`null`] to `driver` because GDI ignores `driver` for
    ///               printer devices.
    ///  * `device` - A pointer to a null-terminated character string that specifies the name of
    ///               the specific output device being used, as shown by the Print Manager (for
    ///               example, Epson FX-80). It is not the printer model name. The `device`
    ///               parameter must be used. To obtain valid names for displays, call
    ///               [`EnumDisplayDevices`]. If `driver` is `DISPLAY` or the device name of a
    ///               specific display device, then `device` must be [`null`] or that same device
    ///               name. If `device` is [`null`], then a DC is created for the primary display
    ///               device. If there are multiple monitors on the system, calling
    ///          `CreateDC(TEXT!("DISPLAY"), std::ptr::null(), std::ptr::null(), std::ptr::null())`
    ///               will create a DC covering all the monitors.
    ///  * `port` - This parameter is ignored and should be set to [`null`]. It is provided only
    ///             for compatibility with 16-bit Windows.
    ///  * `dm` - A pointer to a [`DEVMODE`] structure containing device-specific initialization
    ///           data for the device driver. The [`DocumentProperties`] function retrieves this
    ///           structure filled in for a specified device. The pdm parameter must be [`null`] if
    ///           the device driver is to use the default initialization (if any) specified by the
    ///           user. If `driver` is `DISPLAY`, `dm` must be [`null`]; GDI then uses the display
    ///           device's current [`DEVMODE`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the handle to a DC for the specified device.
    ///
    /// If the function fails, the return value is [`null`].
    ///
    /// # Remarks
    /// Note that the handle to the DC can only be used by a single thread at any one time.
    ///
    /// For parameters `driver` and `device`, call [`EnumDisplayDevices`] to obtain valid names for
    /// displays.
    ///
    /// When you no longer need the DC, call the [`DeleteDC`] function.
    ///
    /// If `driver` or `device` is `DISPLAY`, the thread that calls [`CreateDC`] owns the [`HDC`]
    /// that is created. When this thread is destroyed, the [`HDC`] is no longer valid. Thus, if
    /// you create the [`HDC`] and pass it to another thread, then exit the first thread, the
    /// second thread will not be able to use the [`HDC`].
    ///
    /// When you call [`CreateDC`] to create the [`HDC`] for a display device, you must pass to
    /// `dm` either [`null`] or a pointer to [`DEVMODE`] that matches the current [`DEVMODE`] of
    /// the display device that `device` specifies. We recommend to pass [`null`] and not to try to
    /// exactly match the [`DEVMODE`] for the current display device.
    ///
    /// When you call [`CreateDC`] to create the [`HDC`] for a printer device, the printer driver
    /// validates the [`DEVMODE`]. If the printer driver determines that the [`DEVMODE`] is invalid
    /// (that is, printer driver can’t convert or consume the [`DEVMODE`]), the printer driver
    /// provides a default [`DEVMODE`] to create the [`HDC`] for the printer device.
    ///
    /// ICM: To enable ICM, set the `icm_method` member of the [`DEVMODE`] structure (pointed to by
    /// the `init_data` parameter) to the appropriate value.
    pub unsafe fn CreateDCW(
        driver: LPCWSTR,
        device: LPCWSTR,
        port: LPCWSTR,
        dm: *const DEVMODEW,
    ) -> HDC;
}
