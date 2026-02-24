use crate::{DWORD, dbt::DBT_DEVTYP_PORT};
use std::ffi::c_char;

/// Contains information about a modem, serial, or parallel port.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DEV_BROADCAST_PORT_A {
    /// The size of this structure, in bytes. This is the size of the members plus the actual
    /// length of the `name` string (the null character is accounted for by the declaration of
    /// `name` as a one-character array.)
    pub size: DWORD,

    /// Set to [`DBT_DEVTYP_PORT`].
    pub device_type: DWORD,

    /// Reserved; do not use.
    pub reserved: DWORD,

    /// A null-terminated string specifying the friendly name of the port or the device connected
    /// to the port. Friendly names are intended to help the user quickly and accurately identify
    /// the device—for example, "COM1" and "Standard 28800 bps Modem" are considered friendly
    /// names.
    pub name: [c_char; 1],
}

impl Default for DEV_BROADCAST_PORT_A {
    fn default() -> Self {
        DEV_BROADCAST_PORT_A {
            size: std::mem::size_of::<DEV_BROADCAST_PORT_A>() as _,
            device_type: DBT_DEVTYP_PORT,
            reserved: 0,
            name: [0],
        }
    }
}
