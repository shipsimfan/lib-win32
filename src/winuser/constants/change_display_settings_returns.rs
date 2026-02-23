use crate::LONG;

/// The settings change was successful.
pub const DISP_CHANGE_SUCCESSFUL: LONG = 0;

/// The computer must be restarted for the graphics mode to work.
pub const DISP_CHANGE_RESTART: LONG = 1;

/// The display driver failed the specified graphics mode.
pub const DISP_CHANGE_FAILED: LONG = -1;

/// The graphics mode is not supported.
pub const DISP_CHANGE_BADMODE: LONG = -2;

/// Unable to write settings to the registry.
pub const DISP_CHANGE_NOTUPDATED: LONG = -3;

/// An invalid set of flags was passed in.
pub const DISP_CHANGE_BADFLAGS: LONG = -4;

/// An invalid parameter was passed in. This can include an invalid flag or combination of flags.
pub const DISP_CHANGE_BADPARAM: LONG = -5;

/// The settings change was unsuccessful because the system is DualView capable.
pub const DISP_CHANGE_BADDUALVIEW: LONG = -6;
