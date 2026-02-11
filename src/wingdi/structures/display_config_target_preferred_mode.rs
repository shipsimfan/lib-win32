use crate::{
    DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_TYPE, DISPLAYCONFIG_TARGET_MODE,
    UINT32,
};

/// The [`DISPLAYCONFIG_TARGET_PREFERRED_MODE`] structure contains information about the preferred
/// mode of a display.
#[repr(C)]
#[derive(Clone)]
pub struct DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    /// A [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] structure that contains information about the
    /// request for the target preferred mode. The caller should set the `r#type` member of
    /// [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to
    /// [`DISPLAYCONFIG_DEVICE_INFO_TYPE::GetTargetPreferredMode`] and the `adapter_id` and `id`
    /// members of [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to the target for which the caller wants
    /// the preferred mode. The caller should set the `size` member of
    /// [`DISPLAYCONFIG_DEVICE_INFO_HEADER`] to at least the size of the
    /// [`DISPLAYCONFIG_TARGET_PREFERRED_MODE`] structure.
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,

    /// The width in pixels of the best mode for the monitor that is connected to the target that
    /// the `target_mode` member specifies.
    pub width: UINT32,

    /// The height in pixels of the best mode for the monitor that is connected to the target that
    /// the `target_mode` member specifies.
    pub height: UINT32,

    /// A [`DISPLAYCONFIG_TARGET_MODE`] structure that describes the best target mode for the
    /// monitor that is connected to the specified target.
    pub target_mode: DISPLAYCONFIG_TARGET_MODE,
}

impl Default for DISPLAYCONFIG_TARGET_PREFERRED_MODE {
    fn default() -> Self {
        DISPLAYCONFIG_TARGET_PREFERRED_MODE {
            header: DISPLAYCONFIG_DEVICE_INFO_HEADER {
                r#type: DISPLAYCONFIG_DEVICE_INFO_TYPE::GetTargetPreferredMode,
                size: std::mem::size_of::<DISPLAYCONFIG_TARGET_PREFERRED_MODE>() as _,
                ..Default::default()
            },
            width: 0,
            height: 0,
            target_mode: DISPLAYCONFIG_TARGET_MODE::default(),
        }
    }
}
