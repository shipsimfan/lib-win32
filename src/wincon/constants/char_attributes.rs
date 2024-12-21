use crate::WORD;

/// Text color contains blue.
pub const FOREGROUND_BLUE: WORD = 0x0001;

/// Text color contains green.
pub const FOREGROUND_GREEN: WORD = 0x0002;

/// Text color contains red.
pub const FOREGROUND_RED: WORD = 0x0004;

/// Text color is intensified.
pub const FOREGROUND_INTENSITY: WORD = 0x0008;

/// Background color contains blue.
pub const BACKGROUND_BLUE: WORD = 0x0010;

/// Background color contains green.
pub const BACKGROUND_GREEN: WORD = 0x0020;

/// Background color contains red.
pub const BACKGROUND_RED: WORD = 0x0040;

/// Background color is intensified.
pub const BACKGROUND_INTENSITY: WORD = 0x0080;

/// Leading byte.
pub const COMMON_LVB_LEADING_BYTE: WORD = 0x0100;

/// Trailing byte.
pub const COMMON_LVB_TRAILING_BYTE: WORD = 0x0200;

/// Top horizontal.
pub const COMMON_LVB_GRID_HORIZONTAL: WORD = 0x0400;

/// Left vertical.
pub const COMMON_LVB_GRID_LVERTICAL: WORD = 0x0800;

/// Right vertical.
pub const COMMON_LVB_GRID_RVERTICAL: WORD = 0x1000;

/// Reverse foreground and background attribute.
pub const COMMON_LVB_REVERSE_VIDEO: WORD = 0x4000;

/// Underscore.
pub const COMMON_LVB_UNDERSCORE: WORD = 0x8000;
