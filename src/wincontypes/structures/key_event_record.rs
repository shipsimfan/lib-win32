use crate::{BOOL, DWORD, WCHAR, WORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CAPSLOCK_ON, ENABLE_PROCESSED_INPUT, ENHANCED_KEY, FALSE, INPUT_RECORD, LEFT_ALT_PRESSED,
    LEFT_CTRL_PRESSED, NUMLOCK_ON, RIGHT_ALT_PRESSED, RIGHT_CTRL_PRESSED, SCROLLLOCK_ON,
    SHIFT_PRESSED, TRUE,
};

/// Describes a keyboard input event in a console [`INPUT_RECORD`] structure.
///
/// Enhanced keys for the IBM® 101- and 102-key keyboards are the INS, DEL, HOME, END, PAGE UP,
/// PAGE DOWN, and direction keys in the clusters to the left of the keypad; and the divide (/) and
/// ENTER keys in the keypad.
///
/// Keyboard input events are generated when any key, including control keys, is pressed or
/// released. However, the ALT key when pressed and released without combining with another
/// character, has special meaning to the system and is not passed through to the application.
/// Also, the CTRL+C key combination is not passed through if the input handle is in processed mode
/// ([`ENABLE_PROCESSED_INPUT`]).
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct KEY_EVENT_RECORD {
    /// If the key is pressed, this member is [`TRUE`]. Otherwise, this member is [`FALSE`] (the
    /// key is released).
    pub key_down: BOOL,

    /// The repeat count, which indicates that a key is being held down. For example, when a key is
    /// held down, you might get five events with this member equal to 1, one event with this
    /// member equal to 5, or multiple events with this member greater than or equal to 1.
    pub repeat_count: WORD,

    /// A virtual-key code that identifies the given key in a device-independent manner.
    pub virtual_key_code: WORD,

    /// The virtual scan code of the given key that represents the device-dependent value generated
    /// by the keyboard hardware.
    pub virtual_scan_code: WORD,

    /// Translated Unicode character.
    pub unicode_char: WCHAR,

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
}
