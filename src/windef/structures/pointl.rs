use crate::LONG;

// rustdoc imports
#[allow(unused_imports)]
use crate::POINT;

/// The [`POINTL`] structure defines the x- and y-coordinates of a point.
///
/// # Remarks
/// The [`POINTL`] structure is identical to the [`POINT`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct POINTL {
    /// Specifies the x-coordinate of the point.
    pub x: LONG,

    /// Specifies the y-coordinate of the point.
    pub y: LONG,
}

impl Default for POINTL {
    fn default() -> Self {
        POINTL { x: 0, y: 0 }
    }
}
