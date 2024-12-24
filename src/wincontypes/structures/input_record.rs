use crate::{
    FOCUS_EVENT_RECORD, KEY_EVENT_RECORD, MENU_EVENT_RECORD, MOUSE_EVENT_RECORD,
    WINDOW_BUFFER_SIZE_RECORD, WORD,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    ReadConsoleInput, FOCUS_EVENT, KEY_EVENT, MENU_EVENT, MOUSE_EVENT, WINDOW_BUFFER_SIZE_EVENT,
};

/// Describes an input event in the console input buffer. These records can be read from the input
/// buffer by using the [`ReadConsoleInput`] or [`PeekConsoleInput`] function, or written to the
/// input buffer by using the [`WriteConsoleInput`] function.
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct INPUT_RECORD {
    /// A handle to the type of input event and the event record stored in the `event` member.
    ///
    /// This member can be one of the following values:
    ///  * [`FOCUS_EVENT`] - The `event` member contains a [`FOCUS_EVENT_RECORD`] structure. These
    ///                      events are used internally and should be ignored.
    ///  * [`KEY_EVENT`] - The `event` member contains a [`KEY_EVENT_RECORD`] structure with
    ///                    information about a keyboard event.
    ///  * [`MENU_EVENT`] - The `event` member contains a [`MENU_EVENT_RECORD`] structure. These
    ///                     events are used internally and should be ignored.
    ///  * [`MOUSE_EVENT`] - The `event` member contains a [`MOUSE_EVENT_RECORD`] structure with
    ///                      information about a mouse movement or button press event.
    ///  * [`WINDOW_BUFFER_SIZE_EVENT`] - The `event` member contains a
    ///                                   [`WINDOW_BUFFER_SIZE_RECORD`] structure with information
    ///                                   about the new size of the console screen buffer.
    pub event_type: WORD,

    /// The event information. The format of this member depends on the event type specified by the
    /// `event_type` member.
    pub event: INPUT_RECORD_UNION,
}

#[repr(C)]
#[derive(Clone, Copy)]
#[allow(missing_docs)]
pub union INPUT_RECORD_UNION {
    pub key_event: KEY_EVENT_RECORD,
    pub mouse_event: MOUSE_EVENT_RECORD,
    pub window_buffer_size_event: WINDOW_BUFFER_SIZE_RECORD,
    pub menu_event: MENU_EVENT_RECORD,
    pub focus_event: FOCUS_EVENT_RECORD,
}

impl Default for INPUT_RECORD {
    fn default() -> Self {
        INPUT_RECORD {
            event_type: 0,
            event: INPUT_RECORD_UNION {
                key_event: KEY_EVENT_RECORD::default(),
            },
        }
    }
}
