use crate::{COORD, SMALL_RECT, WORD};

/// Contains information about a console screen buffer.
#[repr(C)]
#[derive(Default, Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    /// A [`COORD`] structure that contains the size of the console screen buffer, in character
    /// columns and rows.
    pub size: COORD,

    /// A [`COORD`] structure that contains the column and row coordinates of the cursor in the
    /// console screen buffer.
    pub cursor_position: COORD,

    /// The attributes of the characters written to a screen buffer by the [`WriteFile`] and
    /// [`WriteConsole`] functions, or echoed to a screen buffer by the [`ReadFile`] and
    /// [`ReadConsole`] functions.
    pub attributes: WORD,

    /// A [`SMALL_RECT`] structure that contains the console screen buffer coordinates of the
    /// upper-left and lower-right corners of the display window.
    pub window: SMALL_RECT,

    /// A [`COORD`] structure that contains the maximum size of the console window, in character columns and rows, given the current screen buffer size and font and the screen size.
    pub maximum_window_size: COORD,
}
