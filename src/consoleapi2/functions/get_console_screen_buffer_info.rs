use crate::{BOOL, HANDLE, PCONSOLE_SCREEN_BUFFER_INFO};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, CONSOLE_SCREEN_BUFFER_INFO, GENERIC_READ};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves information about the specified console screen buffer.
    ///
    /// # Parameters
    ///  * `console_output` - A handle to the console screen buffer. The handle must have the
    ///                       [`GENERIC_READ`] access right.
    ///  * `console_screen_buffer_info` - A pointer to a [`CONSOLE_SCREEN_BUFFER_INFO`] structure
    ///                                   that receives the console screen buffer information.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// The rectangle returned in the `window` member of the [`CONSOLE_SCREEN_BUFFER_INFO`]
    /// structure can be modified and then passed to the [`SetConsoleWindowInfo`] function to
    /// scroll the console screen buffer in the window, to change the size of the window, or both.
    ///
    /// All coordinates returned in the [`CONSOLE_SCREEN_BUFFER_INFO`] structure are in
    /// character-cell coordinates, where the origin (0, 0) is at the upper-left corner of the
    /// console screen buffer.
    pub fn GetConsoleScreenBufferInfo(
        console_output: HANDLE,
        console_screen_buffer_info: PCONSOLE_SCREEN_BUFFER_INFO,
    ) -> BOOL;
}
