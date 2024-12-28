use crate::{BOOL, DWORD, HANDLE, LPDWORD, LPVOID, WCHAR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, SetConsoleMode, WriteConsole, WriteFile, ERROR_NOT_ENOUGH_MEMORY, GENERIC_WRITE,
};
#[allow(unused_imports)]
use std::{ffi::c_char, ptr::null_mut};

#[link(name = "Kernel32")]
extern "system" {
    /// Writes a character string to a console screen buffer beginning at the current cursor
    /// location.
    ///
    /// # Parameters
    ///  * `console_output` - A handle to the console screen buffer. The handle must have the
    ///                       [`GENERIC_WRITE`] access right.
    ///  * `buffer` - A pointer to a buffer that contains characters to be written to the console
    ///               screen buffer. This is expected to be an array of either [`c_char`] for
    ///               [`WriteConsoleA`] or [`WCHAR`] for [`WriteConsoleW`].
    ///  * `number_of_chars_to_write` - The number of characters to be written. If the total size
    ///               of the specified number of characters exceeds the available heap, the
    ///               function fails with [`ERROR_NOT_ENOUGH_MEMORY`].
    ///  * `number_of_chars_written` - A pointer to a variable that receives the number of
    ///                                characters actually written.
    ///  * `reserved` - Reserved; must be [`null_mut`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// The [`WriteConsole`] function writes characters to the console screen buffer at the current
    /// cursor position. The cursor position advances as characters are written. The
    /// [`SetConsoleCursorPosition`] function sets the current cursor position.
    ///
    /// Characters are written using the foreground and background color attributes associated with
    /// the console screen buffer. The [`SetConsoleTextAttribute`] function changes these colors.
    /// To determine the current color attributes and the current cursor position, use
    /// [`GetConsoleScreenBufferInfo`].
    ///
    /// All of the input modes that affect the behavior of the [`WriteFile`] function have the same
    /// effect on [`WriteConsole`]. To retrieve and set the output modes of a console screen
    /// buffer, use the [`GetConsoleMode`] and [`SetConsoleMode`] functions.
    ///
    /// This function uses either Unicode characters or 8-bit characters from the console's current
    /// code page. The console's code page defaults initially to the system's OEM code page. To
    /// change the console's code page, use the [`SetConsoleCP`] or [`SetConsoleOutputCP`]
    /// functions. Legacy consumers may also use the chcp or mode con cp select= commands, but it
    /// is not recommended for new development.
    ///
    /// [`WriteConsole`] fails if it is used with a standard handle that is redirected to a file.
    /// If an application processes multilingual output that can be redirected, determine whether
    /// the output handle is a console handle (one method is to call the [`GetConsoleMode`]
    /// function and check whether it succeeds). If the handle is a console handle, call
    /// [`WriteConsole`]. If the handle is not a console handle, the output is redirected and you
    /// should call [`WriteFile`] to perform the I/O. Be sure to prefix a Unicode plain text file
    /// with a byte order mark.
    ///
    /// Although an application can use [`WriteConsole`] in ANSI mode to write ANSI characters,
    /// consoles do not support "ANSI escape" or "virtual terminal" sequences unless enabled.
    ///
    /// When virtual terminal escape sequences are not enabled, console functions can provide
    /// equivalent functionality. For more information, see [`SetCursorPos`],
    /// [`SetConsoleTextAttribute`], and [`GetConsoleCursorInfo`].
    pub fn WriteConsoleW(
        console_output: HANDLE,
        buffer: *const WCHAR,
        number_of_chars_to_write: DWORD,
        number_of_chars_written: LPDWORD,
        reserved: LPVOID,
    ) -> BOOL;
}
