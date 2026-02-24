use crate::{DWORD, WCHAR, dbt::DBT_DEVTYP_PORT};

/// Contains information about a modem, serial, or parallel port.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DEV_BROADCAST_PORT_W {
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
    pub name: [WCHAR; 1],
}

impl Default for DEV_BROADCAST_PORT_W {
    fn default() -> Self {
        DEV_BROADCAST_PORT_W {
            size: std::mem::size_of::<DEV_BROADCAST_PORT_W>() as _,
            device_type: DBT_DEVTYP_PORT,
            reserved: 0,
            name: [0],
        }
    }
}
