use crate::{POINT, RECT};

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_2::IDXGIOutputDuplication;

/// The [`DXGI_OUTDUPL_MOVE_RECT`] structure describes the movement of a rectangle.
///
/// # Remarks
/// This structure is used by [`IDXGIOutputDuplication::get_frame_move_rects`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_OUTDUPL_MOVE_RECT {
    /// The starting position of a rectangle.
    pub source_point: POINT,

    /// The target region to which to move a rectangle.
    pub destination_rect: RECT,
}

impl Default for DXGI_OUTDUPL_MOVE_RECT {
    fn default() -> Self {
        DXGI_OUTDUPL_MOVE_RECT {
            source_point: POINT::default(),
            destination_rect: RECT::default(),
        }
    }
}
