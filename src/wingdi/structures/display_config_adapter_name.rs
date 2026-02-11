use crate::{DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_TYPE, WCHAR};

/// The [`DISPLAYCONFIG_ADAPTER_NAME`] structure contains information about the display adapter.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DISPLAYCONFIG_ADAPTER_NAME {
    /// A [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure that contains information about the
    /// request for the adapter name. The caller should set the `r#type` member of
    /// [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to [`DISPLAYCONFIG_DEVICE_INFO_TYPE::GetAdapterName`]
    /// and the `adapter_id` member of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to the adapter
    /// identifier of the adapter for which the caller wants the name. For this request, the caller
    /// does not need to set the `id` member of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`]. The caller
    /// should set the `size` member of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to at least the size
    /// of the [`DISPLAYCONFIG_ADAPTER_NAME`] structure.
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,

    /// A NULL-terminated [`WCHAR`] string that is the device name for the adapter. This name can
    /// be used with `SetupAPI.dll` to obtain the device name that is contained in the installation
    /// package.
    pub adapter_device_path: [WCHAR; 128],
}

impl Default for DISPLAYCONFIG_ADAPTER_NAME {
    fn default() -> Self {
        DISPLAYCONFIG_ADAPTER_NAME {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE::GetAdapterName,
                size: std::mem::size_of::<DISPLAYCONFIG_ADAPTER_NAME>() as _,
                ..Default::default()
            },
            adapter_device_path: [0; 128],
        }
    }
}
