use crate::DWORD;

/// HID mouse
pub const MOUSE_HID_HARDWARE: DWORD = 0x0080;

/// HID wheel mouse
pub const WHEELMOUSE_HID_HARDWARE: DWORD = 0x0100;

/// Mouse with horizontal wheel
pub const HORIZONTAL_WHEEL_PRESENT: DWORD = 0x8000;
