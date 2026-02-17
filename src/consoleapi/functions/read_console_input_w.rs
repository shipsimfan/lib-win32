use crate::{BOOL, DWORD, HANDLE, LPDWORD, PINPUT_RECORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GENERIC_READ, GetLastError, INPUT_RECORD};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Reads data from a console input buffer and removes it from the buffer.
    ///
    /// # Parameters
    ///  * `console_input` - A handle to the console input buffer. The handle must have the
    ///                      [`GENERIC_READ`] access right.
    ///  * `buffer` - A pointer to an array of [`INPUT_RECORD`] structures that receives the input
    ///               buffer data.
    ///  * `length` - The size of the array pointed to by the lpBuffer parameter, in array
    ///               elements.
    ///  * `number_of_events_read` - A pointer to a variable that receives the number of input
    ///                              records read.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// If the number of records requested in the nLength parameter exceeds the number of records
    /// available in the buffer, the number available is read. The function does not return until
    /// at least one input record has been read.
    ///
    /// A process can specify a console input buffer handle in one of the wait functions to
    /// determine when there is unread console input. When the input buffer is not empty, the state
    /// of a console input buffer handle is signaled.
    ///
    /// To determine the number of unread input records in a console's input buffer, use the
    /// [`GetNumberOfConsoleInputEvents`] function. To read input records from a console input
    /// buffer without affecting the number of unread records, use the [`PeekConsoleInput`]
    /// function. To discard all unread records in a console's input buffer, use the
    /// [`FlushConsoleInputBuffer`] function.
    ///
    /// This function uses either Unicode characters or 8-bit characters from the console's current
    /// code page. The console's code page defaults initially to the system's OEM code page. To
    /// change the console's code page, use the [`SetConsoleCP`] or [`SetConsoleOutputCP`]
    /// functions. Legacy consumers may also use the chcp or mode con cp select= commands, but it
    /// is not recommended for new development.
    pub fn ReadConsoleInputW(
        console_input: HANDLE,
        buffer: PINPUT_RECORD,
        length: DWORD,
        number_of_events_read: LPDWORD,
    ) -> BOOL;
}
