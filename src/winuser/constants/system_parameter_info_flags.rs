use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::WM_SETTINGCHANGE;

/// Writes the new system-wide parameter setting to the user profile.
pub const SPIF_UPDATEINIFILE: UINT = 0x0001;

/// Broadcasts the [`WM_SETTINGCHANGE`] message after updating the user profile.
pub const SPIF_SENDCHANGE: UINT = 0x0002;

/// Same as [`SPIF_SENDCHANGE`].
pub const SPIF_SENDWININICHANGE: UINT = SPIF_SENDCHANGE;
