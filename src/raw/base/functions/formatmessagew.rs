use crate::raw::{DWord, LPCVoid, LPWStr};
use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{DWordPtr, LocalAlloc, LocalFree, WChar};

#[link(name = "Kernel32")]
extern "C" {
    /// # FormatMessageW function (winbase.h)
    ///
    /// Formats a message string. The function requires a message definition as
    /// input. The message definition can come from a buffer passed into the
    /// function. It can come from a message table resource in an
    /// already-loaded module. Or the caller can ask the function to search the
    /// system's message table resource(s) for the message definition. The
    /// function finds the message definition in a message table resource based
    /// on a message identifier and a language identifier. The function copies
    /// the formatted message text to an output buffer, processing any embedded
    /// insert sequences if requested.
    ///
    /// ## Parameters
    /// `flags`\
    /// The formatting options, and how to interpret the `source` parameter.
    /// The low-order byte of `flags` specifies how the function handles line
    /// breaks in the output buffer. The low-order byte can also specify the
    /// maximum width of a formatted output line.
    ///
    /// This parameter can be one or more of the following values.
    ///  - [`FORMAT_MESSAGE_ALLOCATE_BUFFER`]
    ///  - [`FORMAT_MESSAGE_ARGUMENT_ARRAY`]
    ///  - [`FORMAT_MESSAGE_FROM_HMODULE`]
    ///  - [`FORMAT_MESSAGE_FROM_STRING`]
    ///  - [`FORMAT_MESSAGE_FROM_SYSTEM`]
    ///  - [`FORMAT_MESSAGE_IGNORE_INSERTS`]
    ///
    /// The low-order byte of `flags` can specify the maximum width of a
    /// formatted output line. The following are possible values of the
    /// low-order byte.
    ///  - 0 - There are no output line width restrictions. The function stores
    ///        line breaks that are in the message definition text into the
    ///        output buffer.
    ///  - [`FORMAT_MESSAGE_MAX_WIDTH_MASK`]
    ///
    /// If the low-order byte is a nonzero value other than
    /// [`FORMAT_MESSAGE_MAX_WIDTH_MASK`], it specifies the maximum number of
    /// characters in an output line. The function ignores regular line breaks
    /// in the message definition text. The function never splits a string
    /// delimited by white space across a line break. The function stores
    /// hard-coded line breaks in the message definition text into the output
    /// buffer. Hard-coded line breaks are coded with the %n escape sequence.
    ///
    /// `source`\
    /// The location of the message definition. The type of this parameter
    /// depends upon the settings in the `flags` parameter
    ///
    /// | `flags` Setting                 | Meaning                                                                                                                  |
    /// |---------------------------------|--------------------------------------------------------------------------------------------------------------------------|
    /// | [`FORMAT_MESSAGE_FROM_HMODULE`] | A handle to the module that contains the message table to search.                                                        |
    /// | [`FORMAT_MESSAGE_FROM_STRING`]  | Pointer to a string that consists of unformatted message text. It will be scanned for inserts and formatted accordingly. |
    ///
    /// If neither of these flags is set in `flags`, then `source` is ignored.
    ///
    /// `message_id`\
    /// The message identifier for the requested message. This parameter is
    /// ignored if `flags` includes [`FORMAT_MESSAGE_FROM_STRING`].
    ///
    /// `language_id`\
    /// The language identifier for the requested message. This parameter is
    /// ignored if `flags` includes [`FORMAT_MESSAGE_FROM_STRING`].
    ///
    /// If you pass a specific `LANGID` in this parameter, [`FormatMessageW`]
    /// will return a message for that `LANGID` only. If the function cannot
    /// find a message for that `LANGID`, it sets Last-Error to
    /// [`ERROR_RESOURCE_LANG_NOT_FOUND`]. If you pass in zero,
    /// [`FormatMessageW`] looks for a message for `LANGID`s in the following
    /// order:
    ///  1. Language neutral
    ///  2. Thread `LANGID`, based on the thread's locale value
    ///  3. User default `LANGID`, based on the user's default locale value
    ///  4. System default `LANGID`, based on the system default locale value
    ///  5. US English
    ///
    /// If [`FormatMessageW`] does not locate a message for any of the preceding
    /// `LANGID`s, it returns any language message string that is present. If
    /// that fails, it returns [`ERROR_RESOURCE_LANG_NOT_FOUND`].
    ///
    /// `buffer`\
    /// A pointer to a buffer that receives the null-terminated string that
    /// specifies the formatted message. If `flags` includes
    /// [`FORMAT_MESSAGE_ALLOCATE_BUFFER`], the function allocates a buffer
    /// using the [`LocalAlloc`] function, and places the pointer to the buffer
    /// at the address specified in `buffer`.
    ///
    /// This buffer cannot be larger than 64K bytes.
    ///
    /// `size`\
    /// If the [`FORMAT_MESSAGE_ALLOCATE_BUFFER`] flag is not set, this
    /// parameter specifies the size of the output buffer, in [`WChar`]s. If
    /// [`FORMAT_MESSAGE_ALLOCATE_BUFFER`] is set, this parameter specifies the
    /// minimum number of [`WChar`]s to allocate for an output buffer.
    ///
    /// The output buffer cannot be larger than 64K bytes.
    ///
    /// `arguments`\
    /// An array of values that are used as insert values in the formatted
    /// message. A %1 in the format string indicates the first value in the
    /// Arguments array; a %2 indicates the second argument; and so on.
    ///
    /// The interpretation of each value depends on the formatting information
    /// associated with the insert in the message definition. The default is to
    /// treat each value as a pointer to a null-terminated string.
    ///
    /// By default, the Arguments parameter is of type `va_list*`, which is a
    /// language- and implementation-specific data type for describing a
    /// variable number of arguments. The state of the `va_list` argument is
    /// undefined upon return from the function. To use the `va_list` again,
    /// destroy the variable argument list pointer using `va_end` and
    /// reinitialize it with `va_start`.
    ///
    /// If you do not have a pointer of type `va_list*`, then specify the
    /// [`FORMAT_MESSAGE_ARGUMENT_ARRAY`] flag and pass a pointer to an array
    /// of [`DWordPtr`] values; those values are input to the message formatted
    /// as the insert values. Each insert must have a corresponding element in
    /// the array.
    ///
    /// ## Return Value
    /// If the function succeeds, the return value is the number of [`WChar`]s
    /// stored in the output buffer, excluding the terminating null character.
    ///
    /// If the function fails, the return value is zero. To get extended error
    /// information, call [`GetLastError`].
    pub fn FormatMessageW(
        flags: DWord,
        source: LPCVoid,
        message_id: DWord,
        language_id: DWord,
        buffer: LPWStr,
        size: DWord,
        arguments: *mut c_void,
    ) -> DWord;
}

/// The function ignores regular line breaks in the message definition text.
/// The function stores hard-coded line breaks in the message definition text
/// into the output buffer. The function generates no new line breaks.
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: DWord = 0x000000FF;

/// The function allocates a buffer large enough to hold the formatted message,
/// and places a pointer to the allocated buffer at the address specified by
/// `buffer`. The `buffer` parameter is a pointer to an [`LPTSTR`]. The `size`
/// parameter specifies the minimum number of [`WChar`]s to allocate for an
/// output message buffer. The caller should use the [`LocalFree`] function to
/// free the buffer when it is no longer needed.
///
/// If the length of the formatted message exceeds 128K bytes, then
/// [`FormatMessageW`] will fail and a subsequent call to [`GetLastError`] will
/// return [`ERROR_MORE_DATA`].
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWord = 0x00000100;

/// Insert sequences in the message definition such as %1 are to be ignored and
/// passed through to the output buffer unchanged. This flag is useful for
/// fetching a message for later formatting. If this flag is set, the
/// `arguments` parameter is ignored.
pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWord = 0x00000200;

/// The `source` parameter is a pointer to a null-terminated string that
/// contains a message definition. The message definition may contain insert
/// sequences, just as the message text in a message table resource may. This
/// flag cannot be used with [`FORMAT_MESSAGE_FROM_HMODULE`] or
/// [`FORMAT_MESSAGE_FROM_SYSTEM`].
pub const FORMAT_MESSAGE_FROM_STRING: DWord = 0x00000400;

/// The `source` parameter is a module handle containing the message-table
/// resource(s) to search. If this `source` handle is `NULL`, the current
/// process's application image file will be searched. This flag cannot be used
/// with [`FORMAT_MESSAGE_FROM_STRING`].
///
/// If the module has no message table resource, the function fails with
/// [`ERROR_RESOURCE_TYPE_NOT_FOUND`].
pub const FORMAT_MESSAGE_FROM_HMODULE: DWord = 0x00000800;

/// The function should search the system message-table resource(s) for the
/// requested message. If this flag is specified with
/// [`FORMAT_MESSAGE_FROM_HMODULE`], the function searches the system message
/// table if the message is not found in the module specified by `source`. This
/// flag cannot be used with [`FORMAT_MESSAGE_FROM_STRING`].
///
/// If this flag is specified, an application can pass the result of the
/// [`GetLastError`] function to retrieve the message text for a system-defined
/// error.
pub const FORMAT_MESSAGE_FROM_SYSTEM: DWord = 0x00001000;

/// The Arguments parameter is not a `va_list`` structure, but is a pointer to
/// an array of values that represent the arguments.
///
/// This flag cannot be used with 64-bit integer values. If you are using a
/// 64-bit integer, you must use the `va_list` structure.
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: DWord = 0x00002000;
