//! Type definitions for consoles

mod constants;
mod structures;
mod types;

pub use constants::*;
pub use structures::{
    COORD, FOCUS_EVENT_RECORD, INPUT_RECORD, KEY_EVENT_RECORD, MENU_EVENT_RECORD,
    MOUSE_EVENT_RECORD, SMALL_RECT, WINDOW_BUFFER_SIZE_RECORD,
};
pub use types::PINPUT_RECORD;
