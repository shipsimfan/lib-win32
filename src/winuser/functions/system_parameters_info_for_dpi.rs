use crate::{BOOL, PVOID, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, SystemParametersInfo, SPI_GETICONMETRICS, SPI_GETICONTITLELOGFONT,
    SPI_GETNONCLIENTMETRICS,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Retrieves the value of one of the system-wide parameters, taking into account the provided
    /// DPI value.
    ///
    /// # Parameters
    ///  * `action` - The system-wide parameter to be retrieved. This function is only intended for
    ///               use with [`SPI_GETICONTITLELOGFONT`], [`SPI_GETICONMETRICS`], or
    ///               [`SPI_GETNONCLIENTMETRICS`].
    ///  * `ui_param` - A parameter whose usage and format depends on the system parameter being
    ///                 queried. If not otherwise indicated, you must specify zero for this
    ///                 parameter.
    ///  * `pv_param` - A parameter whose usage and format depends on the system parameter being
    ///                 queried. If not otherwise indicated, you must specify [`null_mut`] for this
    ///                 parameter.
    ///  * `win_ini` - Has no effect for with this API. This parameter only has an effect if you're
    ///                setting parameter.
    ///  * `dpi` - The DPI to use for scaling the metric.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// This function returns a similar result as [`SystemParametersInfo`], but scales it according
    /// to an arbitrary DPI you provide (if appropriate). It only scales with the following
    /// possible values for `action`: [`SPI_GETICONTITLELOGFONT`], [`SPI_GETICONMETRICS`],
    /// [`SPI_GETNONCLIENTMETRICS`]. Other possible `action` values do not provide ForDPI behavior,
    /// and therefore this function returns 0 if called with them.
    ///
    /// For `action` values that contain strings within their associated structures, only Unicode
    /// ([`LOGFONTW`]) strings are supported in this function.
    pub fn SystemParametersInfoForDpi(
        action: UINT,
        ui_param: UINT,
        pv_param: PVOID,
        win_ini: UINT,
        dpi: UINT,
    ) -> BOOL;
}
