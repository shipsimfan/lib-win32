use crate::{LOGFONTA, UINT};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SystemParametersInfo, LOGFONT, NONCLIENTMETRICS, SPI_GETNONCLIENTMETRICS,
    SPI_SETNONCLIENTMETRICS,
};

/// Contains the scalable metrics associated with the nonclient area of a nonminimized window. This
/// structure is used by the [`SPI_GETNONCLIENTMETRICS`] and [`SPI_SETNONCLIENTMETRICS`] actions of
/// the [`SystemParametersInfo`] function.
///
/// # Remarks
/// If the `padded_border_width` member of the [`NONCLIENTMETRICS`] structure is present, this
/// structure is 4 bytes larger than for an application that is compiled with `_WIN32_WINNT` less
/// than or equal to 0x0502.
///
/// Windows Server 2003 and Windows XP/2000:  If an application that is compiled for Windows Server
/// 2008 or Windows Vista must also run on Windows Server 2003 or Windows XP/2000, use the
/// [`GetVersionEx`] function to check the operating system version at run time and, if the
/// application is running on Windows Server 2003 or Windows XP/2000, subtract the size of the
/// `padded_border_width` member from the `size` member of the [`NONCLIENTMETRICS`] structure
/// before calling the [`SystemParametersInfo`] function.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NONCLIENTMETRICSA {
    /// The size of the structure, in bytes. The caller must set this to
    /// `std::mem::size_of::<NONCLIENTMETRICSA>()`.
    pub size: UINT,

    /// The thickness of the sizing border, in pixels. The default is 1 pixel.
    pub border_width: c_int,

    /// The width of a standard vertical scroll bar, in pixels.
    pub scroll_width: c_int,

    /// The height of a standard horizontal scroll bar, in pixels.
    pub scroll_height: c_int,

    /// The width of caption buttons, in pixels.
    pub caption_width: c_int,

    /// The height of caption buttons, in pixels.
    pub caption_height: c_int,

    /// A [`LOGFONT`] structure that contains information about the caption font.
    pub caption_font: LOGFONTA,

    /// The width of small caption buttons, in pixels.
    pub sm_caption_width: c_int,

    /// The height of small captions, in pixels.
    pub sm_caption_height: c_int,

    /// A [`LOGFONT`] structure that contains information about the small caption font.
    pub sm_caption_font: LOGFONTA,

    /// The width of menu-bar buttons, in pixels.
    pub menu_width: c_int,

    /// The height of a menu bar, in pixels.
    pub menu_height: c_int,

    /// A [`LOGFONT`] structure that contains information about the font used in menu bars.
    pub menu_font: LOGFONTA,

    /// A [`LOGFONT`] structure that contains information about the font used in status bars and
    /// tooltips.
    pub status_font: LOGFONTA,

    /// A [`LOGFONT`] structure that contains information about the font used in message boxes.
    pub message_font: LOGFONTA,

    /// The thickness of the padded border, in pixels. The default value is 4 pixels. The
    /// `padded_border_width` and `border_width` members are combined for both resizable and
    /// nonresizable windows in the Windows Aero desktop experience. To compile an application that
    /// uses this member, define `_WIN32_WINNT` as 0x0600 or later.
    pub padded_border_width: c_int,
}

impl Default for NONCLIENTMETRICSA {
    fn default() -> Self {
        NONCLIENTMETRICSA {
            size: std::mem::size_of::<NONCLIENTMETRICSA>() as _,
            border_width: 0,
            scroll_width: 0,
            scroll_height: 0,
            caption_width: 0,
            caption_height: 0,
            caption_font: LOGFONTA::default(),
            sm_caption_width: 0,
            sm_caption_height: 0,
            sm_caption_font: LOGFONTA::default(),
            menu_width: 0,
            menu_height: 0,
            menu_font: LOGFONTA::default(),
            status_font: LOGFONTA::default(),
            message_font: LOGFONTA::default(),
            padded_border_width: 0,
        }
    }
}
