use crate::{HWND, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::USER_DEFAULT_SCREEN_DPI;

#[link(name = "User32")]
unsafe extern "system" {
    /// Returns the dots per inch (dpi) value for the specified window.
    ///
    /// # Parameters
    ///  * `wnd` - The window that you want to get information about.
    ///
    /// # Return Value
    /// The DPI for the window, which depends on the `DPI_AWARENESS` of the window. See the Remarks
    /// section for more information. An invalid `wnd` value will result in a return value of 0.
    ///
    /// # Remarks
    /// The following indicates the return value of [`GetDpiForWindow`] based on the
    /// `DPI_AWARENESS` of the provided `wnd`:
    ///  * [`DPI_AWARENESS_UNAWARE`] - The base value of DPI is which is set to 96 (defined as
    ///                                [`USER_DEFAULT_SCREEN_DPI`])
    ///  * [`DPI_AWARENESS_SYSTEM_AWARE`] - The system DPI.
    ///  * [`DPI_AWARENESS_PER_MONITOR_AWARE`] - The DPI of the monitor where the window is
    ///                                          located.
    pub fn GetDpiForWindow(wnd: HWND) -> UINT;
}
