use crate::{DWORD, USHORT};

/// Defines the raw input data coming from the specified Human Interface Device (HID).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct RID_DEVICE_INFO_HID {
    /// The vendor identifier for the HID.
    pub vendor_id: DWORD,

    /// The product identifier for the HID.
    pub product_id: DWORD,

    /// The version number for the HID.
    pub version_number: DWORD,

    /// The top-level collection Usage Page for the device.
    pub usage_page: USHORT,

    /// The top-level collection Usage for the device.
    pub usage: USHORT,
}

impl Default for RID_DEVICE_INFO_HID {
    fn default() -> Self {
        RID_DEVICE_INFO_HID {
            vendor_id: 0,
            product_id: 0,
            version_number: 0,
            usage_page: 0,
            usage: 0,
        }
    }
}
