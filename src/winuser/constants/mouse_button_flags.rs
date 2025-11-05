use crate::USHORT;

/// Left button changed to down.
pub const RI_MOUSE_BUTTON_1_DOWN: USHORT = 0x0001;

/// Left button changed to down.
pub const RI_MOUSE_LEFT_BUTTON_DOWN: USHORT = 0x0001;

/// Left button changed to up.
pub const RI_MOUSE_BUTTON_1_UP: USHORT = 0x0002;

/// Left button changed to up.
pub const RI_MOUSE_LEFT_BUTTON_UP: USHORT = 0x0002;

/// Right button changed to down.
pub const RI_MOUSE_BUTTON_2_DOWN: USHORT = 0x0004;

/// Right button changed to down.
pub const RI_MOUSE_RIGHT_BUTTON_DOWN: USHORT = 0x0004;

/// Right button changed to up.
pub const RI_MOUSE_BUTTON_2_UP: USHORT = 0x0008;

/// Right button changed to up.
pub const RI_MOUSE_RIGHT_BUTTON_UP: USHORT = 0x0008;

/// Middle button changed to down.
pub const RI_MOUSE_BUTTON_3_DOWN: USHORT = 0x0010;

/// Middle button changed to down.
pub const RI_MOUSE_MIDDLE_BUTTON_DOWN: USHORT = 0x0010;

/// Middle button changed to up.
pub const RI_MOUSE_BUTTON_3_UP: USHORT = 0x0020;

/// Middle button changed to up.
pub const RI_MOUSE_MIDDLE_BUTTON_UP: USHORT = 0x0020;

/// XBUTTON1 changed to down.
pub const RI_MOUSE_BUTTON_4_DOWN: USHORT = 0x0040;

/// XBUTTON1 changed to up.
pub const RI_MOUSE_BUTTON_4_UP: USHORT = 0x0080;

/// XBUTTON2 changed to down.
pub const RI_MOUSE_BUTTON_5_DOWN: USHORT = 0x0100;

/// XBUTTON2 changed to up.
pub const RI_MOUSE_BUTTON_5_UP: USHORT = 0x0200;

/// Raw input comes from a mouse wheel. The wheel delta is stored in `button_data`. A positive
/// value indicates that the wheel was rotated forward, away from the user; a negative value
/// indicates that the wheel was rotated backward, toward the user.
pub const RI_MOUSE_WHEEL: USHORT = 0x0400;

/// Raw input comes from a horizontal mouse wheel. The wheel delta is stored in `button_data`. A
/// positive value indicates that the wheel was rotated to the right; a negative value indicates
/// that the wheel was rotated to the left.
pub const RI_MOUSE_HWHEEL: USHORT = 0x0800;
