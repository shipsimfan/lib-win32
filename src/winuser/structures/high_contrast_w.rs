use crate::{DWORD, LPTSTR, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SystemParametersInfo, SystemParametersInfoW, HCF_AVAILABLE, HCF_CONFIRMHOTKEY,
    HCF_HIGHCONTRASTON, HCF_HOTKEYACTIVE, HCF_HOTKEYAVAILABLE, HCF_HOTKEYSOUND, HCF_INDICATOR,
    HCF_OPTION_NOTHEMECHANGE, HIGHCONTRAST, SPI_GETHIGHCONTRAST, SPI_SETHIGHCONTRAST,
    WM_THEMECHANGED,
};

/// Contains information about the high contrast accessibility feature. This feature sets the
/// appearance scheme of the user interface for maximum visibility for a visually-impaired user,
/// and advises applications to comply with this appearance scheme.
///
/// # Remarks
/// An application uses this structure when calling the [`SystemParametersInfoW`] function with the
/// [`SPI_GETHIGHCONTRAST`] or [`SPI_SETHIGHCONTRAST`] value. When using [`SPI_GETHIGHCONTRAST`],
/// an application must specify the `size` member of the [`HIGHCONTRAST`] structure; the
/// [`SystemParametersInfo`] function fills the remaining members. An application must specify all
/// structure members when using the [`SPI_SETHIGHCONTRAST`] value.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct HIGHCONTRASTW {
    /// Specifies the size, in bytes, of this structure.
    pub size: UINT,

    /// Specifies a combination of the following values:
    ///  * [`HCF_HIGHCONTRASTON`] - The high contrast feature is on.
    ///  * [`HCF_AVAILABLE`] - The high contrast feature is available.
    ///  * [`HCF_HOTKEYACTIVE`] - The user can turn the high contrast feature on and off by
    ///                           simultaneously pressing the left ALT, left SHIFT, and PRINT
    ///                           SCREEN keys.
    ///  * [`HCF_CONFIRMHOTKEY`] - A confirmation dialog appears when the high contrast feature is
    ///                            activated by using the hot key.
    ///  * [`HCF_HOTKEYSOUND`] - A siren is played when the user turns the high contrast feature on
    ///                          or off by using the hot key.
    ///  * [`HCF_INDICATOR`] -  A visual indicator is displayed when the high contrast feature is
    ///                         on. This value is not currently used and is ignored.
    ///  * [`HCF_HOTKEYAVAILABLE`] -  The hot key associated with the high contrast feature can be
    ///                               enabled. An application can retrieve this value, but cannot
    ///                               set it.
    ///  * [`HCF_OPTION_NOTHEMECHANGE`] - Passing [`HIGHCONTRAST`] structure in calls to
    ///                                   [`SystemParametersInfoW`] can cause theme change effects
    ///                                   even if the theme isn't being changed. For example, the
    ///                                   [`WM_THEMECHANGED`] message is sent to Windows even if
    ///                                   the only change is to [`HCF_HOTKEYSOUND`]. To prevent
    ///                                   this, include the [`HCF_OPTION_NOTHEMECHANGE`] flag in
    ///                                   the call to [`SystemParametersInfo`].
    pub flags: DWORD,

    /// Points to a string that contains the name of the color scheme that will be set to the
    /// default scheme.
    pub default_scheme: LPTSTR,
}
