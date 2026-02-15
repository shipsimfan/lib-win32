use crate::{DISPLAYCONFIG_PIXELFORMAT, POINTL, UINT32};

/// The [`DISPLAYCONFIG_SOURCE_MODE`] structure represents a point or an offset in a
/// two-dimensional space.
///
/// # Remarks
/// The arrangement of source surfaces on the desktop is controlled by the position member, which
/// specifies the position in desktop coordinates of the upper-left corner of the source surface.
/// The source surface that is positioned at (0, 0) is considered the primary. GDI has strict rules
/// about how the source surfaces can be arranged in the desktop space. For example, there cannot
/// be any gaps between source surfaces, and there can be no overlaps.
///
/// The [`SetDisplayConfig`] function attempts to rearrange source surfaces in order to enforce
/// these layout rules. The caller must make every effort to lay out the source surfaces correctly
/// because GDI rearranges the sources in an undefined manner to enforce the layout rules. The
/// resultant layout may not be what the caller wanted to achieve.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DISPLAYCONFIG_SOURCE_MODE {
    /// The width in pixels of the source mode.
    pub width: UINT32,

    /// The height in pixels of the source mode.
    pub height: UINT32,

    /// A value from the [`DISPLAYCONFIG_PIXELFORMAT`] enumeration that specifies the pixel format
    /// of the source mode.
    pub pixel_format: DISPLAYCONFIG_PIXELFORMAT,

    /// A [`POINTL`] structure that specifies the position in the desktop coordinate space of the
    /// upper-left corner of this source surface. The source surface that is located at (0, 0) is
    /// always the primary source surface.
    pub position: POINTL,
}

impl Default for DISPLAYCONFIG_SOURCE_MODE {
    fn default() -> Self {
        DISPLAYCONFIG_SOURCE_MODE {
            width: 0,
            height: 0,
            pixel_format: DISPLAYCONFIG_PIXELFORMAT::_8Bpp,
            position: POINTL::default(),
        }
    }
}
