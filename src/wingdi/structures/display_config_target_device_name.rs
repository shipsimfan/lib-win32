use crate::{
    DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_TYPE,
    DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS, DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY, UINT16, UINT32,
    WCHAR,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::DisplayConfigGetDeviceInfo;

/// The [`DISPLAYCONFIG_TARGET_DEVICE_NAME`] structure contains information about the target.
///
/// # Remarks
/// Extended display identification data (EDID) is a set of data that is provided by a display to
/// describe its capabilities to a graphics adapter. EDID data allows a computer to detect the type
/// of monitor that is connected to it. EDID data includes the manufacturer name, the product type,
/// the timings that are supported by the display, the display size, as well as other display
/// characteristics. EDID is defined by a standard published by the Video Electronics Standards
/// Association (VESA).
///
/// A named device object has a path and name of the form `\Device\DeviceName`. This is known as
/// the device name of the device object.
///
/// If an application calls the [`DisplayConfigGetDeviceInfo`] function to obtain the monitor name
/// and [`DisplayConfigGetDeviceInfo`] either cannot get the monitor name or the target is forced
/// without a monitor connected, the string in the `monitor_friendly_device_name` member of the
/// [`DISPLAYCONFIG_TARGET_DEVICE_NAME`] structure is a NULL string and none of the bit-field flags
/// in the [`DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS`] structure are set.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME {
    /// A [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure that contains information about the
    /// request for the target device name. The caller should set the `r#type` member of
    /// [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to [`DISPLAYCONFIG_DEVICE_INFO_TYPE::GetTargetName`]
    /// and the `adapter_id` and `id` members of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to the target
    /// for which the caller wants the target device name. The caller should set the `size` member
    /// of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to at least the size of the
    /// [`DISPLAYCONFIG_TARGET_DEVICE_NAME`] structure.
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,

    /// A [`DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS`] structure that identifies, in bit-field flags,
    /// information about the target.
    pub flags: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS,

    /// A value from the [`DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY`] enumeration that specifies the
    /// target's connector type.
    pub output_technology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,

    /// The manufacture identifier from the monitor extended display identification data (EDID).
    /// This member is set only when the `edid_ids_valid` bit-field is set in the `flags` member.
    pub edid_manufacture_id: UINT16,

    /// The product code from the monitor EDID. This member is set only when the `edid_ids_valid`
    /// bit-field is set in the `flags` member.
    pub edid_product_code_id: UINT16,

    /// The one-based instance number of this particular target only when the adapter has multiple
    /// targets of this type. The connector instance is a consecutive one-based number that is
    /// unique within each adapter. If this is the only target of this type on the adapter, this
    /// value is zero.
    pub connector_instance: UINT32,

    /// A NULL-terminated [`WCHAR`] string that is the device name for the monitor. This name can
    /// be used with `SetupAPI.dll` to obtain the device name that is contained in the installation
    /// package.
    pub monitor_friendly_device_name: [WCHAR; 64],

    /// A NULL-terminated [`WCHAR`] string that is the device name for the monitor. This name can
    /// be used with `SetupAPI.dll` to obtain the device name that is contained in the installation
    /// package.
    pub monitor_device_path: [WCHAR; 128],
}

impl Default for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    fn default() -> Self {
        DISPLAYCONFIG_TARGET_DEVICE_NAME {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE::GetTargetName,
                size: std::mem::size_of::<DISPLAYCONFIG_TARGET_DEVICE_NAME>() as _,
                ..Default::default()
            },
            flags: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS::default(),
            output_technology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::Other,
            edid_manufacture_id: 0,
            edid_product_code_id: 0,
            connector_instance: 0,
            monitor_friendly_device_name: [0; 64],
            monitor_device_path: [0; 128],
        }
    }
}
