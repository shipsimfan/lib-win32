use crate::ULONG;

/// Include interlaced modes.
pub const DXGI_ENUM_MODES_INTERLACED: ULONG = 1;

/// Include stretched-scaling modes.
pub const DXGI_ENUM_MODES_SCALING: ULONG = 2;

/// Include stereo modes.
///
/// Direct3D 11: This enumeration value is supported starting with Windows 8.
pub const DXGI_ENUM_MODES_STEREO: ULONG = 4;

/// Include stereo modes that are hidden because the user has disabled stereo. Control panel
/// applications can use this option to show stereo capabilities that have been disabled as part of
/// a user interface that enables and disables stereo.
///
/// Direct3D 11: This enumeration value is supported starting with Windows 8.
pub const DXGI_ENUM_MODES_DISABLED_STEREO: ULONG = 8;
