use crate::LONG;

// rustdoc imports
#[allow(unused_imports)]
use crate::RECT;

/// The [`RECTL`] structure defines a rectangle by the coordinates of its upper-left and
/// lower-right corners.
///
/// # Remarks
/// The [`RECTL`] structure is identical to the [`RECT`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RECTL {
    /// Specifies the x-coordinate of the upper-left corner of the rectangle.
    pub left: LONG,

    /// Specifies the y-coordinate of the upper-left corner of the rectangle.
    pub top: LONG,

    /// Specifies the x-coordinate of the lower-right corner of the rectangle.
    pub right: LONG,

    /// Specifies the y-coordinate of the lower-right corner of the rectangle.
    pub bottom: LONG,
}

impl Default for RECTL {
    fn default() -> Self {
        RECTL {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}
