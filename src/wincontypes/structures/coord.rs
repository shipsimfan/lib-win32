use crate::SHORT;

/// Defines the coordinates of a character cell in a console screen buffer. The origin of the
/// coordinate -system (0,0) is at the top, left cell of the buffer.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct COORD {
    /// The horizontal coordinate or column value. The units depend on the function call.
    pub x: SHORT,

    /// The vertical coordinate or row value. The units depend on the function call.
    pub y: SHORT,
}

impl Default for COORD {
    fn default() -> Self {
        COORD { x: 0, y: 0 }
    }
}
