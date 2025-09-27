use crate::{BOOL, PVOID, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, SystemParametersInfoForDpi, HIGHCONTRAST, MAKELANGID, SPIF_SENDCHANGE,
    SPIF_SENDWININICHANGE, SPIF_UPDATEINIFILE, SPI_GETHIGHCONTRAST, SPI_SETHIGHCONTRAST,
    WM_LBUTTONUP, WM_SETTINGCHANGE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Retrieves or sets the value of one of the system-wide parameters. This function can also
    /// update the user profile while setting a parameter.
    ///
    /// # Parameters
    ///  * `action` - The system-wide parameter to be retrieved or set.
    ///  * `ui_param` - A parameter whose usage and format depends on the system parameter being
    ///                 queried or set. If not otherwise indicated, you must specify zero for this
    ///                 parameter.
    ///  * `pv_param` - A parameter whose usage and format depends on the system parameter being
    ///                 queried or set. If not otherwise indicated, you must specify [`null_mut`]
    ///                 for this parameter.
    ///  * `win_ini` - If a system parameter is being set, specifies whether the user profile is to
    ///                be updated, and if so, whether the [`WM_SETTINGCHANGE`] message is to be
    ///                broadcast to all top-level windows to notify them of the change. This
    ///                parameter can be zero if you do not want to update the user profile or
    ///                broadcast the [`WM_SETTINGCHANGE`] message, or it can be one or more of the
    ///                following values:
    ///    * [`SPIF_UPDATEINIFILE`] -  Writes the new system-wide parameter setting to the user
    ///                                profile.
    ///    * [`SPIF_SENDCHANGE`] -  Broadcasts the [`WM_SETTINGCHANGE`] message after updating the
    ///                             user profile.
    ///    * [`SPIF_SENDWININICHANGE`] -  Same as [`SPIF_SENDCHANGE`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a nonzero value.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// This function is intended for use with applications that allow the user to customize the
    /// environment.
    ///
    /// A keyboard layout name should be derived from the hexadecimal value of the language
    /// identifier corresponding to the layout. For example, U.S. English has a language identifier
    /// of 0x0409, so the primary U.S. English layout is named "00000409". Variants of U.S. English
    /// layout, such as the Dvorak layout, are named "00010409", "00020409" and so on. For a list
    /// of the primary language identifiers and sublanguage identifiers that make up a language
    /// identifier, see the [`MAKELANGID`] macro.
    ///
    /// There is a difference between the High Contrast color scheme and the High Contrast Mode.
    /// The High Contrast color scheme changes the system colors to colors that have obvious
    /// contrast; you switch to this color scheme by using the Display Options in the control
    /// panel. The High Contrast Mode, which uses [`SPI_GETHIGHCONTRAST`] and
    /// [`SPI_SETHIGHCONTRAST`], advises applications to modify their appearance for
    /// visually-impaired users. It involves such things as audible warning to users and
    /// customized color scheme (using the Accessibility Options in the control panel). For
    /// more information, see [`HIGHCONTRAST`].
    ///
    /// During the time that the primary button is held down to activate the Mouse "ClickLock"
    /// feature, the user can move the mouse. After the primary button is locked down, releasing
    /// the primary button does not result in a [`WM_LBUTTONUP`] message. Thus, it will appear to
    /// an application that the primary button is still down. Any subsequent button message
    /// releases the primary button, sending a [`WM_LBUTTONUP`] message to the application, thus
    /// the button can be unlocked programmatically or through the user clicking any button.
    ///
    /// This API is not DPI aware, and should not be used if the calling thread is per-monitor DPI
    /// aware. For the DPI-aware version of this API, see [`SystemParametersInfoForDpi`].
    pub fn SystemParametersInfoA(
        action: UINT,
        ui_param: UINT,
        pv_param: PVOID,
        win_ini: UINT,
    ) -> BOOL;
}
