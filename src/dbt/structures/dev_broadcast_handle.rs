use crate::{BYTE, DWORD, GUID, HANDLE, HDEVNOTIFY, LONG, dbt::DBT_DEVTYP_HANDLE};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::{RegisterDeviceNotification, dbt::DBT_CUSTOMEVENT};

/// Contains information about a file system handle.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DEV_BROADCAST_HANDLE {
    /// The size of this structure, in bytes.
    pub size: DWORD,

    /// Set to [`DBT_DEVTYP_HANDLE`].
    pub device_type: DWORD,

    /// Reserved; do not use.
    pub reserved: DWORD,

    /// A handle to the device to be checked.
    pub handle: HANDLE,

    /// A handle to the device notification. This handle is returned by
    /// [`RegisterDeviceNotification`].
    pub dev_notify: HDEVNOTIFY,

    /// The [`GUID`] for the custom event. Valid only for [`DBT_CUSTOMEVENT`].
    pub event_guid: GUID,

    /// The offset of an optional string buffer. Valid only for [`DBT_CUSTOMEVENT`].
    pub name_offset: LONG,

    /// Optional binary data. This member is valid only for [`DBT_CUSTOMEVENT`].
    pub data: [BYTE; 1],
}

impl Default for DEV_BROADCAST_HANDLE {
    fn default() -> Self {
        DEV_BROADCAST_HANDLE {
            size: std::mem::size_of::<DEV_BROADCAST_HANDLE>() as _,
            device_type: DBT_DEVTYP_HANDLE,
            reserved: 0,
            handle: null_mut(),
            dev_notify: null_mut(),
            event_guid: GUID::default(),
            name_offset: 0,
            data: [0],
        }
    }
}
