use crate::{
    CCHDEVICENAME, DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_TYPE, WCHAR,
};

/// The [`DISPLAYCONFIG_SOURCE_DEVICE_NAME`] structure contains the GDI device name for the source
/// or view.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    /// A [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure that contains information about the
    /// request for the source device name. The caller should set the `r#type` member of
    /// [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to [`DISPLAYCONFIG_DEVICE_INFO_TYPE::GetSourceName`]
    /// and the `adapter_id` and `id` members of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to the source
    /// for which the caller wants the source device name. The caller should set the `size` member
    /// of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to at least the size of the
    /// [`DISPLAYCONFIG_SOURCE_DEVICE_NAME`] structure.
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,

    /// A NULL-terminated [`WCHAR`] string that is the GDI device name for the source, or view.
    /// This name can be used in a call to [`EnumDisplaySettings`] to obtain a list of available
    /// modes for the specified source.
    pub view_gdi_device_name: [WCHAR; CCHDEVICENAME],
}

impl Default for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    fn default() -> Self {
        DISPLAYCONFIG_SOURCE_DEVICE_NAME {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE::GetSourceName,
                size: std::mem::size_of::<DISPLAYCONFIG_SOURCE_DEVICE_NAME>() as _,
                ..Default::default()
            },
            view_gdi_device_name: [0; CCHDEVICENAME],
        }
    }
}
