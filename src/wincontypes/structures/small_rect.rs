use crate::SHORT;

/// Defines the coordinates of the upper left and lower right corners of a rectangle.
///
/// This structure is used by console functions to specify rectangular areas of console screen
/// buffers, where the coordinates specify the rows and columns of screen-buffer character cells.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SMALL_RECT {
    /// The x-coordinate of the upper left corner of the rectangle.
    pub left: SHORT,

    /// The y-coordinate of the upper left corner of the rectangle.
    pub top: SHORT,

    /// The x-coordinate of the lower right corner of the rectangle.
    pub right: SHORT,

    /// The y-coordinate of the lower right corner of the rectangle.
    pub bottom: SHORT,
}

impl Default for SMALL_RECT {
    fn default() -> Self {
        SMALL_RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }
}
