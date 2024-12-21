use crate::COORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::ENABLE_WINDOW_INPUT;

/// Describes a change in the size of the console screen buffer.
///
/// Buffer size events are placed in the input buffer when the console is in window-aware mode
/// ([`ENABLE_WINDOW_INPUT`]).
#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct WINDOW_BUFFER_SIZE_RECORD {
    /// A [`COORD`] structure that contains the size of the console screen buffer, in character
    /// cell columns and rows.
    pub size: COORD,
}
