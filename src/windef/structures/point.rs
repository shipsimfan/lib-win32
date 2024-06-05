use crate::LONG;

/// The [`POINT`] structure defines the x- and y-coordinates of a point.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct POINT {
    /// Specifies the x-coordinate of the point.
    pub x: LONG,

    /// Specifies the y-coordinate of the point.
    pub y: LONG,
}

impl Default for POINT {
    fn default() -> Self {
        POINT { x: 0, y: 0 }
    }
}
