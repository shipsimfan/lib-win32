use crate::{BOOL, BYTE, HDC, RECT};
use std::ptr::null_mut;

/// The [`PAINTSTRUCT`] structure contains information for an application. This information can be
/// used to paint the client area of a window owned by that application.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct PAINTSTRUCT {
    /// A handle to the display DC to be used for painting.
    pub hdc: HDC,

    /// Indicates whether the background must be erased. This value is nonzero if the application
    /// should erase the background. The application is responsible for erasing the background if a
    /// window class is created without a background brush.
    pub erase: BOOL,

    /// A [`RECT`] structure that specifies the upper left and lower right corners of the rectangle
    /// in which the painting is requested, in device units relative to the upper-left corner of
    /// the client area.
    pub paint: RECT,

    /// Reserved; used internally by the system.
    pub restore: BOOL,

    /// Reserved; used internally by the system.
    pub inc_update: BOOL,

    /// Reserved; used internally by the system.
    pub rgb_reserved: [BYTE; 32],
}

impl Default for PAINTSTRUCT {
    fn default() -> Self {
        PAINTSTRUCT {
            hdc: null_mut(),
            erase: 0,
            paint: RECT::default(),
            restore: 0,
            inc_update: 0,
            rgb_reserved: [0; 32],
        }
    }
}
