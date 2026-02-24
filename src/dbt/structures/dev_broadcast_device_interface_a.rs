use crate::{DWORD, GUID, dbt::DBT_DEVTYP_DEVICEINTERFACE};
use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::{RegisterDeviceNotificationA, RegisterDeviceNotificationW, WM_DEVICECHANGE};

/// Contains information about a class of devices.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DEV_BROADCAST_DEVICEINTERFACE_A {
    /// The size of this structure, in bytes. This is the size of the members plus the actual
    /// length of the `name` string (the null character is accounted for by the declaration of
    /// `name` as a one-character array.)
    pub size: DWORD,

    /// Set to [`DBT_DEVTYP_DEVICEINTERFACE`].
    pub device_type: DWORD,

    /// Reserved; do not use.
    pub reserved: DWORD,

    /// The [`GUID`] for the interface device class.
    pub class_guid: GUID,

    /// A null-terminated string that specifies the name of the device.
    ///
    /// When this structure is returned to a window through the [`WM_DEVICECHANGE`] message, the
    /// `name` string is converted to ANSI as appropriate. Services always receive a Unicode
    /// string, whether they call [`RegisterDeviceNotificationW`] or
    /// [`RegisterDeviceNotificationA`].
    pub name: [c_char; 1],
}

impl Default for DEV_BROADCAST_DEVICEINTERFACE_A {
    fn default() -> Self {
        DEV_BROADCAST_DEVICEINTERFACE_A {
            size: std::mem::size_of::<DEV_BROADCAST_DEVICEINTERFACE_A>() as _,
            device_type: DBT_DEVTYP_DEVICEINTERFACE,
            reserved: 0,
            class_guid: GUID::default(),
            name: [0],
        }
    }
}
