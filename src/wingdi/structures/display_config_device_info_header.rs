use crate::{DISPLAYCONFIG_DEVICE_INFO_TYPE, LUID, UINT32};

// rustdoc imports
#[allow(unused_imports)]
use crate::{DisplayConfigGetDeviceInfo, DisplayConfigSetDeviceInfo};

/// The [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure contains display information about the
/// device.
///
/// # Remarks
/// The [`DisplayConfigGetDeviceInfo`] function uses the [`DISPLAYCONFIG_DEVICE_INFO_HEADER`]
/// structure for retrieving display configuration information about the device, and the
/// [`DisplayConfigSetDeviceInfo`] function uses the [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure
/// for setting display configuration information for the device.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DISPLAYCONFIG_DEVICE_INFO_HEADER {
    /// A [`DISPLAYCONFIG_DEVICE_INFO_TYPE`] enumerated value that determines the type of device
    /// information to retrieve or set. The remainder of the packet for the retrieve or set
    /// operation follows immediately after the [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure.
    pub r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE,

    /// The size, in bytes, of the device information that is retrieved or set. This size includes
    /// the size of the header and the size of the additional data that follows the header. This
    /// device information depends on the request type.
    pub size: UINT32,

    /// A locally unique identifier ([`LUID`]) that identifies the adapter that the device
    /// information packet refers to.
    pub adapter_id: LUID,

    /// The source or target identifier to get or set the device information for. The meaning of
    /// this identifier is related to the type of information being requested. For example, in the
    /// case of [`DISPLAYCONFIG_DEVICE_INFO_TYPE::GetSourceName`], this is the source identifier.
    pub id: UINT32,
}

impl Default for DISPLAYCONFIG_DEVICE_INFO_HEADER {
    fn default() -> Self {
        DISPLAYCONFIG_DEVICE_INFO_HEADER {
            r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE::GetSourceName,
            size: 0,
            adapter_id: LUID::default(),
            id: 0,
        }
    }
}
