use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    WM_DEVICECHANGE,
    dbt::{
        _DEV_BROADCAST_USERDEFINED, DBT_DEVTYP_DEVICEINTERFACE, DBT_DEVTYP_HANDLE, DBT_DEVTYP_OEM,
        DBT_DEVTYP_PORT, DBT_DEVTYP_VOLUME, DEV_BROADCAST_DEVICEINTERFACE, DEV_BROADCAST_HANDLE,
        DEV_BROADCAST_HDR, DEV_BROADCAST_OEM, DEV_BROADCAST_PORT, DEV_BROADCAST_VOLUME,
    },
};

/// Serves as a standard header for information related to a device event reported through the
/// [`WM_DEVICECHANGE`] message.
///
/// The members of the [`DEV_BROADCAST_HDR`] structure are contained in each device management
/// structure. To determine which structure you have received through [`WM_DEVICECHANGE`], treat
/// the structure as a [`DEV_BROADCAST_HDR`] structure and check its `device_type` member.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct _DEV_BROADCAST_HDR {
    /// The size of this structure, in bytes.
    ///
    /// If this is a user-defined event, this member must be the size of this header, plus the size
    /// of the variable-length data in the [`_DEV_BROADCAST_USERDEFINED`] structure.
    pub size: DWORD,

    /// The device type, which determines the event-specific information that follows the first
    /// three members. This member can be one of the following values:
    ///  * [`DBT_DEVTYP_DEVICEINTERFACE`] - Class of devices. This structure is a
    ///                                     [`DEV_BROADCAST_DEVICEINTERFACE`] structure.
    ///  * [`DBT_DEVTYP_HANDLE`] - File system handle. This structure is a [`DEV_BROADCAST_HANDLE`]
    ///                            structure.
    ///  * [`DBT_DEVTYP_OEM`] - OEM- or IHV-defined device type. This structure is a
    ///                         [`DEV_BROADCAST_OEM`] structure.
    ///  * [`DBT_DEVTYP_PORT`] - Port device (serial or parallel). This structure is a
    ///                          [`DEV_BROADCAST_PORT`] structure.
    ///  * [`DBT_DEVTYP_VOLUME`] -  Logical volume. This structure is a [`DEV_BROADCAST_VOLUME`]
    ///                             structure.
    pub device_type: DWORD,

    /// Reserved; do not use.
    pub reserved: DWORD,
}

impl Default for _DEV_BROADCAST_HDR {
    fn default() -> Self {
        _DEV_BROADCAST_HDR {
            size: 0,
            device_type: 0,
            reserved: 0,
        }
    }
}
