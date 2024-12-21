use crate::{WCHAR, WORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    BACKGROUND_BLUE, BACKGROUND_GREEN, BACKGROUND_INTENSITY, BACKGROUND_RED,
    COMMON_LVB_GRID_HORIZONTAL, COMMON_LVB_GRID_LVERTICAL, COMMON_LVB_GRID_RVERTICAL,
    COMMON_LVB_LEADING_BYTE, COMMON_LVB_REVERSE_VIDEO, COMMON_LVB_TRAILING_BYTE,
    COMMON_LVB_UNDERSCORE, FOREGROUND_BLUE, FOREGROUND_GREEN, FOREGROUND_INTENSITY, FOREGROUND_RED,
};

/// Specifies a Unicode or ANSI character and its attributes. This structure is used by console
/// functions to read from and write to a console screen buffer.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct CHAR_INFO {
    /// Unicode character of a screen buffer character cell.
    pub char: WCHAR,

    /// The character attributes. This member can be zero or any combination of the following
    /// values:
    ///  * [`FOREGROUND_BLUE`] - Text color contains blue.
    ///  * [`FOREGROUND_GREEN`] - Text color contains green.
    ///  * [`FOREGROUND_RED`] - Text color contains red.
    ///  * [`FOREGROUND_INTENSITY`] - Text color is intensified.
    ///  * [`BACKGROUND_BLUE`] - Background color contains blue.
    ///  * [`BACKGROUND_GREEN`] - Background color contains green.
    ///  * [`BACKGROUND_RED`] - Background color contains red.
    ///  * [`BACKGROUND_INTENSITY`] - Background color is intensified.
    ///  * [`COMMON_LVB_LEADING_BYTE`] - Leading byte.
    ///  * [`COMMON_LVB_TRAILING_BYTE`] - Trailing byte.
    ///  * [`COMMON_LVB_GRID_HORIZONTAL`] - Top horizontal.
    ///  * [`COMMON_LVB_GRID_LVERTICAL`] - Left vertical.
    ///  * [`COMMON_LVB_GRID_RVERTICAL`] - Right vertical.
    ///  * [`COMMON_LVB_REVERSE_VIDEO`] - Reverse foreground and background attribute.
    ///  * [`COMMON_LVB_UNDERSCORE`] - Underscore.
    pub attributes: WORD,
}
