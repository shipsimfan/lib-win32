use crate::{
    DISPLAYCONFIG_DESKTOP_IMAGE_INFO, DISPLAYCONFIG_MODE_INFO_TYPE, DISPLAYCONFIG_SOURCE_MODE,
    DISPLAYCONFIG_TARGET_MODE, LUID, UINT32,
};
use std::ops::{Deref, DerefMut};

/// The [`DISPLAYCONFIG_MODE_INFO`] structure contains either source mode or target mode
/// information.
#[repr(C)]
#[derive(Clone)]
pub struct DISPLAYCONFIG_MODE_INFO {
    /// A value that indicates whether the [`DISPLAYCONFIG_MODE_INFO`] structure represents source
    /// or target mode information. If `info_type` is [`DISPLAYCONFIG_MODE_INFO_TYPE::Target`], the
    /// `target_mode` parameter value contains a valid [`DISPLAYCONFIG_TARGET_MODE`] structure
    /// describing the specified target. If `info_type` is
    /// [`DISPLAYCONFIG_MODE_INFO_TYPE::Source`], the sourceMode parameter value contains a valid
    /// [`DISPLAYCONFIG_SOURCE_MODE`] structure describing the specified source.
    pub info_type: DISPLAYCONFIG_MODE_INFO_TYPE,

    /// The source or target identifier on the specified adapter that this path relates to.
    pub id: UINT32,

    /// The identifier of the adapter that this source or target mode information relates to.
    pub adapter_id: LUID,

    #[allow(missing_docs)]
    pub dummy: DISPLAYCONFIG_MODE_INFO_UNION,
}

impl Default for DISPLAYCONFIG_MODE_INFO {
    fn default() -> Self {
        DISPLAYCONFIG_MODE_INFO {
            info_type: DISPLAYCONFIG_MODE_INFO_TYPE::Source,
            id: 0,
            adapter_id: LUID::default(),
            dummy: DISPLAYCONFIG_MODE_INFO_UNION::default(),
        }
    }
}

impl Deref for DISPLAYCONFIG_MODE_INFO {
    type Target = DISPLAYCONFIG_MODE_INFO_UNION;

    fn deref(&self) -> &Self::Target {
        &self.dummy
    }
}

impl DerefMut for DISPLAYCONFIG_MODE_INFO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dummy
    }
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPLAYCONFIG_MODE_INFO_UNION {
    /// A valid [`DISPLAYCONFIG_TARGET_MODE`] structure that describes the specified target only
    /// when `info_type` is [`DISPLAYCONFIG_MODE_INFO_TYPE::Target`].
    pub target_mode: DISPLAYCONFIG_TARGET_MODE,

    /// A valid [`DISPLAYCONFIG_SOURCE_MODE`] structure that describes the specified source only
    /// when `info_type` is [`DISPLAYCONFIG_MODE_INFO_TYPE::Source`].
    pub source_mode: DISPLAYCONFIG_SOURCE_MODE,

    /// A [`DISPLAYCONFIG_DESKTOP_IMAGE_INFO`] structure that describes information about the
    /// desktop image only when `info_type` is [`DISPLAYCONFIG_MODE_INFO_TYPE::DesktopImage`].
    ///
    /// Supported starting in Windows 10.
    pub desktop_image_info: DISPLAYCONFIG_DESKTOP_IMAGE_INFO,
}

impl Default for DISPLAYCONFIG_MODE_INFO_UNION {
    fn default() -> Self {
        DISPLAYCONFIG_MODE_INFO_UNION {
            target_mode: DISPLAYCONFIG_TARGET_MODE::default(),
        }
    }
}
