use crate::DWORD;

/// The right ALT key is pressed.
pub const RIGHT_ALT_PRESSED: DWORD = 0x0001;

/// The left ALT key is pressed.
pub const LEFT_ALT_PRESSED: DWORD = 0x0002;

/// The right CTRL key is pressed.
pub const RIGHT_CTRL_PRESSED: DWORD = 0x0004;

/// The left CTRL key is pressed.
pub const LEFT_CTRL_PRESSED: DWORD = 0x0008;

/// The SHIFT key is pressed
pub const SHIFT_PRESSED: DWORD = 0x0010;

/// The NUM LOCK light is on.
pub const NUMLOCK_ON: DWORD = 0x0020;

/// The SCROLL LOCK light is on.
pub const SCROLLLOCK_ON: DWORD = 0x0040;

/// The CAPS LOCK light is on.
pub const CAPSLOCK_ON: DWORD = 0x0080;

/// The key is enhanced.
pub const ENHANCED_KEY: DWORD = 0x0100;
