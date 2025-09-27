use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SystemParametersInfo, HIGHCONTRAST, WM_THEMECHANGED};

/// The high contrast feature is on.
pub const HCF_HIGHCONTRASTON: DWORD = 0x00000001;

/// The high contrast feature is available.
pub const HCF_AVAILABLE: DWORD = 0x00000002;

/// The user can turn the high contrast feature on and off by simultaneously pressing the left ALT,
/// left SHIFT, and PRINT SCREEN keys.
pub const HCF_HOTKEYACTIVE: DWORD = 0x00000004;

/// A confirmation dialog appears when the high contrast feature is activated by using the hot key.
pub const HCF_CONFIRMHOTKEY: DWORD = 0x00000008;

/// A siren is played when the user turns the high contrast feature on or off by using the hot key.
pub const HCF_HOTKEYSOUND: DWORD = 0x00000010;

/// A visual indicator is displayed when the high contrast feature is on. This value is not
/// currently used and is ignored.
pub const HCF_INDICATOR: DWORD = 0x00000020;

/// The hot key associated with the high contrast feature can be enabled. An application can
/// retrieve this value, but cannot set it.
pub const HCF_HOTKEYAVAILABLE: DWORD = 0x00000040;

/// Passing [`HIGHCONTRAST`] structure in calls to [`SystemParametersInfo`] can cause theme change
/// effects even if the theme isn't being changed. For example, the [`WM_THEMECHANGED`] message is
/// sent to Windows even if the only change is to [`HCF_HOTKEYSOUND`].
///
/// To prevent this, include the [`HCF_OPTION_NOTHEMECHANGE`] flag in the call to
/// [`SystemParametersInfo`].
pub const HCF_OPTION_NOTHEMECHANGE: DWORD = 0x00001000;
