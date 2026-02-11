use crate::{
    DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_TYPE,
    DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::DisplayConfigGetDeviceInfo;

/// Specifies base output technology info for a given target ID.
///
/// # Remarks
/// For a Miracast display device, a call to the [`DisplayConfigGetDeviceInfo`] function always
/// returns a value of [`DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::Miracast`], regardless of what the
/// Miracast sink reports as the connector type.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DISPLAYCONFIG_TARGET_BASE_TYPE {
    /// A [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure that contains info about the request for
    /// the target device name. The caller should set the `r#type` member of
    /// [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to
    /// [`DISPLAYCONFIG_DEVICE_INFO_TYPE::GetTargetBaseType`] and the `adapter_id` and `id` members
    /// of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to the target for which the caller wants the target
    /// device name.
    ///
    /// The caller should set the `size` member of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to at least
    /// the size of the [`DISPLAYCONFIG_TARGET_BASE_TYPE`] structure.
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,

    /// The base output technology, given as a constant value of the
    /// [`DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY`] enumeration, of the adapter and the target
    /// specified by the header member.
    pub base_output_technology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
}

impl Default for DISPLAYCONFIG_TARGET_BASE_TYPE {
    fn default() -> Self {
        DISPLAYCONFIG_TARGET_BASE_TYPE {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE::GetTargetBaseType,
                size: std::mem::size_of::<DISPLAYCONFIG_TARGET_BASE_TYPE>() as _,
                ..Default::default()
            },
            base_output_technology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY::Other,
        }
    }
}
