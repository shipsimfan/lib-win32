use crate::DWORD;

/// Raw input comes from the mouse.
pub const RIM_TYPEMOUSE: DWORD = 0;

/// Raw input comes from the keyboard.
pub const RIM_TYPEKEYBOARD: DWORD = 1;

/// Raw input comes from some device that is not a keyboard or a mouse.
pub const RIM_TYPEHID: DWORD = 2;
