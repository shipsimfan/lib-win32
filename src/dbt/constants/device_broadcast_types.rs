use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::dbt::{
    DEV_BROADCAST_DEVICEINTERFACE, DEV_BROADCAST_HANDLE, DEV_BROADCAST_OEM, DEV_BROADCAST_PORT,
    DEV_BROADCAST_VOLUME,
};

/// Class of devices. This structure is a [`DEV_BROADCAST_DEVICEINTERFACE`] structure.
pub const DBT_DEVTYP_DEVICEINTERFACE: DWORD = 0x00000005;

/// File system handle. This structure is a [`DEV_BROADCAST_HANDLE`] structure.
pub const DBT_DEVTYP_HANDLE: DWORD = 0x00000006;

/// OEM- or IHV-defined device type. This structure is a [`DEV_BROADCAST_OEM`] structure.
pub const DBT_DEVTYP_OEM: DWORD = 0x00000000;

/// Port device (serial or parallel). This structure is a [`DEV_BROADCAST_PORT`] structure.
pub const DBT_DEVTYP_PORT: DWORD = 0x00000003;

/// Logical volume. This structure is a [`DEV_BROADCAST_VOLUME`] structure.
pub const DBT_DEVTYP_VOLUME: DWORD = 0x00000002;
