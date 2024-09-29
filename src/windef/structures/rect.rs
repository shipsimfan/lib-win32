use crate::LONG;

/// The [`RECT`] structure defines a rectangle by the coordinates of its upper-left and lower-right
/// corners.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RECT {
    /// Specifies the x-coordinate of the upper-left corner of the rectangle.
    pub left: LONG,

    /// Specifies the y-coordinate of the upper-left corner of the rectangle.
    pub top: LONG,

    /// Specifies the x-coordinate of the lower-right corner of the rectangle.
    pub right: LONG,

    /// Specifies the y-coordinate of the lower-right corner of the rectangle.
    pub bottom: LONG,
}

impl Default for RECT {
    fn default() -> Self {
        RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}
