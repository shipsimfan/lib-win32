use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::dbt::DBT_DEVTYP_DEVICEINTERFACE;

/// The `recipient` parameter is a window handle.
pub const DEVICE_NOTIFY_WINDOW_HANDLE: DWORD = 0x00000000;

/// The `recipient` parameter is a service status handle.
pub const DEVICE_NOTIFY_SERVICE_HANDLE: DWORD = 0x00000001;

/// Notifies the recipient of device interface events for all device interface classes. (The
/// `class_guid` member is ignored.)
///
/// This value can be used only if the `device_type` member is [`DBT_DEVTYP_DEVICEINTERFACE`].
pub const DEVICE_NOTIFY_ALL_INTERFACE_CLASSES: DWORD = 0x00000004;
