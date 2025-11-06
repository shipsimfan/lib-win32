use crate::SHORT;

// rustdoc imports
#[allow(unused_imports)]
use crate::{LONG, POINT, POINTL};

/// The [`POINTS`] structure defines the x- and y-coordinates of a point.
///
/// # Remarks
/// The [`POINTS`] structure is similar to the [`POINT`] and [`POINTL`] structures. The difference
/// is that the members of the [`POINTS`] structure are of type [`SHORT`], while those of the other
/// two structures are of type [`LONG`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct POINTS {
    /// Specifies the x-coordinate of the point.
    pub x: SHORT,

    /// Specifies the y-coordinate of the point.
    pub y: SHORT,
}

impl Default for POINTS {
    fn default() -> Self {
        POINTS { x: 0, y: 0 }
    }
}
