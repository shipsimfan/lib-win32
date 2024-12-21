use crate::{COORD, DWORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CAPSLOCK_ON, ENABLE_MOUSE_INPUT, ENHANCED_KEY, FROM_LEFT_1ST_BUTTON_PRESSED,
    FROM_LEFT_2ND_BUTTON_PRESSED, FROM_LEFT_3RD_BUTTON_PRESSED, FROM_LEFT_4TH_BUTTON_PRESSED,
    INPUT_RECORD, LEFT_ALT_PRESSED, LEFT_CTRL_PRESSED, NUMLOCK_ON, RIGHTMOST_BUTTON_PRESSED,
    RIGHT_ALT_PRESSED, RIGHT_CTRL_PRESSED, SCROLLLOCK_ON, SHIFT_PRESSED,
};

/// Describes a mouse input event in a console [`INPUT_RECORD`] structure.
///
/// Mouse events are placed in the input buffer when the console is in mouse mode
/// ([`ENABLE_MOUSE_INPUT`]).
///
/// Mouse events are generated whenever the user moves the mouse, or presses or releases one of the
/// mouse buttons. Mouse events are placed in a console's input buffer only when the console group
/// has the keyboard focus and the cursor is within the borders of the console's window.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct MOUSE_EVENT_RECORD {
    /// A [`COORD`] structure that contains the location of the cursor, in terms of the console
    /// screen buffer's character-cell coordinates.
    pub mouse_position: COORD,

    /// The status of the mouse buttons. The least significant bit corresponds to the leftmost
    /// mouse button. The next least significant bit corresponds to the rightmost mouse button. The
    /// next bit indicates the next-to-leftmost mouse button. The bits then correspond left to
    /// right to the mouse buttons. A bit is 1 if the button was pressed.
    ///
    /// The following constants are defined for the first five mouse buttons:
    ///  * [`FROM_LEFT_1ST_BUTTON_PRESSED`] - The leftmost button.
    ///  * [`FROM_LEFT_2ND_BUTTON_PRESSED`] - The second button from the left.
    ///  * [`FROM_LEFT_3RD_BUTTON_PRESSED`] - The third button from the left.
    ///  * [`FROM_LEFT_4TH_BUTTON_PRESSED`] - The fourth button from the left.
    ///  * [`RIGHTMOST_BUTTON_PRESSED`] - The rightmost mouse button.
    pub button_state: DWORD,

    /// The state of the control keys. This member can be one or more of the following values:
    ///  * [`CAPSLOCK_ON`] - The CAPS LOCK light is on.
    ///  * [`ENHANCED_KEY`] - The key is enchanced.
    ///  * [`LEFT_ALT_PRESSED`] - The left ALT key is pressed.
    ///  * [`LEFT_CTRL_PRESSED`] - The left CTRL key is pressed.
    ///  * [`NUMLOCK_ON`] - The NUM LOCK light is on.
    ///  * [`RIGHT_ALT_PRESSED`] - The right ALT key is pressed.
    ///  * [`RIGHT_CTRL_PRESSED`] - The right CTRL key is pressed.
    ///  * [`SCROLLLOCK_ON`] - The SCROLL LOCK light is on.
    ///  * [`SHIFT_PRESSED`] - The SHIFT key is pressed.
    pub control_key_state: DWORD,

    /// The type of mouse event. If this value is zero, it indicates a mouse button being pressed
    /// or released. Otherwise, this member is one of the following values:
    ///  * [`DOUBLE_CLICK`] - The second click (button press) of a double-click occurred. The first
    ///                       click is returned as a regular button-press event.
    ///  * [`MOUSE_HWHEELED`] - The horizontal mouse wheel was moved. If the high word of the
    ///                         `button_state` member contains a positive value, the wheel was
    ///                         rotated to the right. Otherwise, the wheel was rotated to the left.
    ///  * [`MOUSE_MOVED`] - A change in mouse position occurred.
    ///  * [`MOUSE_WHEELED`] - The vertical mouse wheel was moved. If the high word of the
    ///                        `button_state` member contains a positive value, the wheel was
    ///                        rotated forward, away from the user. Otherwise, the wheel was
    ///                        rotated backward, toward the user.
    pub event_flags: DWORD,
}
