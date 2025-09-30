use crate::{POINT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_2::{
    IDXGIOutputDuplication, DXGI_OUTDUPL_POINTER_POSITION, DXGI_OUTDUPL_POINTER_SHAPE_TYPE,
};

/// The [`DXGI_OUTDUPL_POINTER_SHAPE_INFO`] structure describes information about the cursor shape.
///
/// # Remarks
/// An application draws the cursor shape with the top-left-hand corner drawn at the position that
/// the `position` member of the [`DXGI_OUTDUPL_POINTER_POSITION`] structure specifies; the
/// application does not use the hot spot to draw the cursor shape.
///
/// An application calls the [`IDXGIOutputDuplication::get_frame_pointer_shape`] method to retrieve
/// cursor shape information in a [`DXGI_OUTDUPL_POINTER_SHAPE_INFO`] structure.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    /// A [`DXGI_OUTDUPL_POINTER_SHAPE_TYPE`]-typed value that specifies the type of cursor shape.
    pub r#type: UINT,

    /// The width in pixels of the mouse cursor.
    pub width: UINT,

    /// The height in scan lines of the mouse cursor.
    pub height: UINT,

    /// The width in bytes of the mouse cursor.
    pub pitch: UINT,

    /// The position of the cursor's hot spot relative to its upper-left pixel. An application does
    /// not use the hot spot when it determines where to draw the cursor shape.
    pub hot_spot: POINT,
}

impl Default for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn default() -> Self {
        DXGI_OUTDUPL_POINTER_SHAPE_INFO {
            r#type: 0,
            width: 0,
            height: 0,
            pitch: 0,
            hot_spot: POINT::default(),
        }
    }
}
