use crate::{DWORD, dbt::DBT_DEVTYP_OEM};

/// Contains information about a OEM-defined device type.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DEV_BROADCAST_OEM {
    /// The size of this structure, in bytes.
    pub size: DWORD,

    /// Set to [`DBT_DEVTYP_OEM`].
    pub device_type: DWORD,

    /// Reserved; do not use.
    pub reserved: DWORD,

    /// The OEM-specific identifier for the device.
    pub identifier: DWORD,

    /// The OEM-specific function value. Possible values depend on the device.
    pub suppfunc: DWORD,
}

impl Default for DEV_BROADCAST_OEM {
    fn default() -> Self {
        DEV_BROADCAST_OEM {
            size: std::mem::size_of::<DEV_BROADCAST_OEM>() as _,
            device_type: DBT_DEVTYP_OEM,
            reserved: 0,
            identifier: 0,
            suppfunc: 0,
        }
    }
}
