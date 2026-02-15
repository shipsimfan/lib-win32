use crate::{POINTL, RECTL};

/// The [`DISPLAYCONFIG_DESKTOP_IMAGE_INFO`] structure contains information about the image
/// displayed on the desktop.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    /// A [`POINTL`] structure that specifies the size of the VidPn source surface that is being
    /// displayed on the monitor.
    pub path_source_size: POINTL,

    /// A [`RECTL`] structure that defines where the desktop image will be positioned within path
    /// source. Region must be completely inside the bounds of the path source size.
    pub desktop_image_region: RECTL,

    /// A [`RECTL`] structure that defines which part of the desktop image for this clone group
    /// will be displayed on this path. This currently must be set to the desktop size.
    pub desktop_image_clip: RECTL,
}

impl Default for DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
    fn default() -> Self {
        DISPLAYCONFIG_DESKTOP_IMAGE_INFO {
            path_source_size: POINTL::default(),
            desktop_image_region: RECTL::default(),
            desktop_image_clip: RECTL::default(),
        }
    }
}
