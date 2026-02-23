use crate::DWORD;

/// The graphics mode for the current screen will be changed dynamically and the graphics mode will
/// be updated in the registry. The mode information is stored in the `USER` profile.
pub const CDS_UPDATEREGISTRY: DWORD = 0x00000001;

/// The system tests if the requested graphics mode could be set.
pub const CDS_TEST: DWORD = 0x00000002;

/// The mode is temporary in nature.
///
/// If you change to and from another desktop, this mode will not be reset.
pub const CDS_FULLSCREEN: DWORD = 0x00000004;

/// The settings will be saved in the global settings area so that they will affect all users on
/// the machine. Otherwise, only the settings for the user are modified. This flag is only valid
/// when specified with the [`CDS_UPDATEREGISTRY`] flag.
pub const CDS_GLOBAL: DWORD = 0x00000008;

/// This device will become the primary device.
pub const CDS_SET_PRIMARY: DWORD = 0x00000010;

/// When set, the lParam parameter is a pointer to a [`VIDEOPARAMETERS`] structure.
pub const CDS_VIDEOPARAMETERS: DWORD = 0x00000020;

/// Enables settings changes to unsafe graphics modes.
pub const CDS_ENABLE_UNSAFE_MODES: DWORD = 0x00000100;

/// Disables settings changes to unsafe graphics modes.
pub const CDS_DISABLE_UNSAFE_MODES: DWORD = 0x00000200;

/// The settings should be changed, even if the requested settings are the same as the current
/// settings.
pub const CDS_RESET: DWORD = 0x40000000;

#[allow(missing_docs)]
pub const CDS_RESET_EX: DWORD = 0x20000000;

/// The settings will be saved in the registry, but will not take effect. This flag is only valid
/// when specified with the [`CDS_UPDATEREGISTRY`] flag.
pub const CDS_NORESET: DWORD = 0x10000000;
