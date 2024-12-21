use crate::DWORD;

/// A change in mouse position occurred.
pub const MOUSE_MOVED: DWORD = 0x0001;

/// The second click (button press) of a double-click occurred. The first click is returned as a
/// regular button-press event.
pub const DOUBLE_CLICK: DWORD = 0x0002;

/// The vertical mouse wheel was moved.
///
/// If the high word of the `button_state` member contains a positive value, the wheel was rotated
/// forward, away from the user. Otherwise, the wheel was rotated backward, toward the user.
pub const MOUSE_WHEELED: DWORD = 0x0004;

/// The horizontal mouse wheel was moved.
///
/// If the high word of the `button_state` member contains a positive value, the wheel was rotated
/// to the right. Otherwise, the wheel was rotated to the left.
pub const MOUSE_HWHEELED: DWORD = 0x0008;
