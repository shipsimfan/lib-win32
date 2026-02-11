use crate::UINT32;

// rustdoc imports
#[allow(unused_imports)]
use crate::DISPLAYCONFIG_TARGET_DEVICE_NAME;

/// The [`DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS`] structure contains information about a target
/// device.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    /// # Bit 0
    /// A [`UINT32`] value that indicates that the string in the `monitor_friendly_device_name`
    /// member of the [`DISPLAYCONFIG_TARGET_DEVICE_NAME`] structure was constructed from the
    /// manufacture identification string in the extended display identification data (EDID).
    ///
    /// Setting this member is equivalent to setting the first bit of the 32-bit `value` member
    /// (0x00000001).
    ///
    /// # Bit 1
    /// A [`UINT32`] value that indicates that the target is forced with no detectable monitor
    /// attached and the `monitor_friendly_device_name` member of the
    /// [`DISPLAYCONFIG_TARGET_DEVICE_NAME`] structure is a NULL-terminated empty string.
    ///
    /// Setting this member is equivalent to setting the second bit of the 32-bit `value` member
    /// (0x00000002).
    ///
    /// # Bit 2
    /// A [`UINT32`] value that indicates that the edid_manufacture_id and `edid_product_code_id`
    /// members of the [`DISPLAYCONFIG_TARGET_DEVICE_NAME`] structure are valid and were obtained
    /// from the EDID.
    ///
    /// Setting this member is equivalent to setting the third bit of the 32-bit `value` member
    /// (0x00000004).
    pub value: UINT32,
}

impl Default for DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS {
    fn default() -> Self {
        DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS { value: 0 }
    }
}
