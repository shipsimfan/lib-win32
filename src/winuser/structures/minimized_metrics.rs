use crate::UINT;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SystemParametersInfo, ARW_BOTTOMLEFT, ARW_BOTTOMRIGHT, ARW_DOWN, ARW_HIDE, ARW_LEFT, ARW_RIGHT,
    ARW_TOPLEFT, ARW_TOPRIGHT, ARW_UP, SPI_GETMINIMIZEDMETRICS, SPI_SETMINIMIZEDMETRICS,
};

/// Contains the scalable metrics associated with minimized windows. This structure is used with
/// the [`SystemParametersInfo`] function when the [`SPI_GETMINIMIZEDMETRICS`] or
/// [`SPI_SETMINIMIZEDMETRICS`] action value is specified.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MINIMIZEDMETRICS {
    /// The size of the structure, in bytes. The caller must set this to
    /// `std::mem::size_of::<MINIMIZEDMETRICS>()`.
    pub size: UINT,

    /// The width of minimized windows, in pixels.
    pub width: c_int,

    /// The horizontal space between arranged minimized windows, in pixels.
    pub horz_gap: c_int,

    /// The vertical space between arranged minimized windows, in pixels.
    pub vert_gap: c_int,

    /// The starting position and direction used when arranging minimized windows. The starting
    /// position must be one of the following values:
    ///  * [`ARW_BOTTOMLEFT`] - Start at the lower-left corner of the work area.
    ///  * [`ARW_BOTTOMRIGHT`] - Start at the lower-right corner of the work area.
    ///  * [`ARW_TOPLEFT`] - Start at the upper-left corner of the work area.
    ///  * [`ARW_TOPRIGHT`] - Start at the upper-right corner of the work area.
    ///
    /// The direction must be one of the following values:
    ///  * [`ARW_LEFT`] - Arrange left (valid with [`ARW_BOTTOMRIGHT`] and [`ARW_TOPRIGHT`] only).
    ///  * [`ARW_RIGHT`] - Arrange right (valid with [`ARW_BOTTOMLEFT`] and [`ARW_TOPLEFT`] only).
    ///  * [`ARW_UP`] - Arrange up (valid with [`ARW_BOTTOMLEFT`] and ARW_BOTTOMRIGHT only).
    ///  * [`ARW_DOWN`] - Arrange down (valid with [`ARW_TOPLEFT`] and [`ARW_TOPRIGHT`] only).
    ///  * [`ARW_HIDE`] - Hide minimized windows by moving them off the visible area of the screen.
    pub arrange: c_int,
}
