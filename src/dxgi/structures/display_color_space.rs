use crate::FLOAT;

/// Don't use this structure; it is not supported and it will be removed from the header in a
/// future release.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_DISPLAY_COLOR_SPACE {
    /// The primary coordinates, as an 8 by 2 array of [`FLOAT`] values.
    pub primary_coordinates: [[FLOAT; 2]; 8],

    /// The white points, as a 16 by 2 array of [`FLOAT`] values.
    pub white_points: [[FLOAT; 2]; 16],
}

impl Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        DXGI_DISPLAY_COLOR_SPACE {
            primary_coordinates: [[0.; 2]; 8],
            white_points: [[0.; 2]; 16],
        }
    }
}
