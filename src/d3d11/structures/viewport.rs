use crate::FLOAT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{D3D11_VIEWPORT_BOUNDS_MAX, D3D11_VIEWPORT_BOUNDS_MIN};

/// Defines the dimensions of a viewport.
///
/// # Remarks
/// In all cases, `width` and `height` must be `>= 0` and `top_left_x + width` and
/// `top_left_y + height` must be `<= D3D11_VIEWPORT_BOUNDS_MAX`.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_VIEWPORT {
    /// X position of the left hand side of the viewport. Ranges between
    /// [`D3D11_VIEWPORT_BOUNDS_MIN`] and [`D3D11_VIEWPORT_BOUNDS_MAX`].
    pub top_left_x: FLOAT,

    /// Y position of the top of the viewport. Ranges between [`D3D11_VIEWPORT_BOUNDS_MIN`] and
    /// [`D3D11_VIEWPORT_BOUNDS_MAX`].
    pub top_left_y: FLOAT,

    /// Width of the viewport.
    pub width: FLOAT,

    /// Height of the viewport.
    pub height: FLOAT,

    /// Minimum depth of the viewport. Ranges between 0 and 1.
    pub min_depth: FLOAT,

    /// Maximum depth of the viewport. Ranges between 0 and 1.
    pub max_depth: FLOAT,
}

impl Default for D3D11_VIEWPORT {
    fn default() -> Self {
        D3D11_VIEWPORT {
            top_left_x: 0.,
            top_left_y: 0.,
            width: 0.,
            height: 0.,
            min_depth: 0.,
            max_depth: 1.,
        }
    }
}
