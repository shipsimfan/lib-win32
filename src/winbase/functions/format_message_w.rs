use crate::{DWORD, LPCVOID, LPWSTR};
use std::ffi::VaList;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, LocalAlloc, DWORD_PTR, ERROR_RESOURCE_LANG_NOT_FOUND,
    FORMAT_MESSAGE_ALLOCATE_BUFFER, FORMAT_MESSAGE_ARGUMENT_ARRAY, FORMAT_MESSAGE_FROM_HMODULE,
    FORMAT_MESSAGE_FROM_STRING, FORMAT_MESSAGE_FROM_SYSTEM, FORMAT_MESSAGE_IGNORE_INSERTS, LANGID,
    TCHAR,
};

#[link(name = "Kernel32")]
extern "system" {
    /// Formats a message string. The function requires a message definition as input. The message
    /// definition can come from a buffer passed into the function. It can come from a message
    /// table resource in an already-loaded module. Or the caller can ask the function to search
    /// the system's message table resource(s) for the message definition. The function finds the
    /// message definition in a message table resource based on a message identifier and a language
    /// identifier. The function copies the formatted message text to an output buffer, processing
    /// any embedded insert sequences if requested.
    ///
    /// # Parameters
    ///  * `flags` - The formatting options, and how to interpret the `source` parameter. The
    ///              low-order byte of `flags` specifies how the function handles line breaks in
    ///              the output buffer. The low-order byte can also specify the maximum width of a
    ///              formatted output line. This parameter can be one or more of the following
    ///              values:
    ///    * [`FORMAT_MESSAGE_ALLOCATE_BUFFER`] - The function allocates a buffer large enough to
    ///                                           hold the formatted message, and places a pointer
    ///                                           to the allocated buffer at the address specified
    ///                                           by `buffer`.
    ///    * [`FORMAT_MESSAGE_ARGUMENT_ARRAY`] - The `arguments` parameter is not a [`VaList`]
    ///                                          structure, but is a pointer to an array of values
    ///                                          that represent the arguments.
    ///    * [`FORMAT_MESSAGE_FROM_HMODULE`] - The `source` parameter is a module handle containing
    ///                                        the message-table resource(s) to search.
    ///    * [`FORMAT_MESSAGE_FROM_STRING`] - The `source` parameter is a pointer to a
    ///                                       null-terminated string that contains a message
    ///                                       definition.
    ///    * [`FORMAT_MESSAGE_FROM_SYSTEM`] - The function should search the system message-table
    ///                                       resource(s) for the requested message.
    ///    * [`FORMAT_MESSAGE_IGNORE_INSERTS`] - Insert sequences in the message definition such as
    ///                                          `%1` are to be ignored and passed through to the
    ///                                          output buffer unchanged.
    ///  * `source` - The location of the message definition. The type of this parameter depends
    ///               upon the settings in the `flags` parameter.
    ///  * `message_id` - The message identifier for the requested message. This parameter is
    ///                   ignored if `flags` includes [`FORMAT_MESSAGE_FROM_STRING`].
    ///  * `language_id` - The language identifier for the requested message. This parameter is
    ///                    ignored if `flags` includes [`FORMAT_MESSAGE_FROM_STRING`]. If you pass
    ///                    a specific [`LANGID`] in this parameter, [`FormatMessage`] will return a
    ///                    message for that [`LANGID`] only. If the function cannot find a message
    ///                    for that [`LANGID`], it sets Last-Error to
    ///                    [`ERROR_RESOURCE_LANG_NOT_FOUND`]. If you pass in zero,
    ///                    [`FormatMessage`] looks for a message for [`LANGID`]s in the following
    ///                    order:
    ///    1. Language neutral
    ///    2. Thread [`LANGID`], based on the thread's locale value
    ///    3. User default [`LANGID`], based on the user's default locale value
    ///    4. System default [`LANGID`], based on the system default locale value
    ///    5. US English
    ///  * `buffer` - A pointer to a buffer that receives the null-terminated string that specifies
    ///               the formatted message. If `flags` includes
    ///               [`FORMAT_MESSAGE_ALLOCATE_BUFFER`], the function allocates a buffer using the
    ///               [`LocalAlloc`] function, and places the pointer to the buffer at the address
    ///               specified in `buffer`. This buffer cannot be larger than 64K bytes.
    ///  * `size` - If the [`FORMAT_MESSAGE_ALLOCATE_BUFFER`] flag is not set, this parameter
    ///             specifies the size of the output buffer, in [`TCHAR`]s. If
    ///             [`FORMAT_MESSAGE_ALLOCATE_BUFFER`] is set, this parameter specifies the minimum
    ///             number of [`TCHAR`]s to allocate for an output buffer. The output buffer cannot
    ///             be larger than 64K bytes.
    ///  * `arguments` - An array of values that are used as insert values in the formatted
    ///                  message. A `%1` in the format string indicates the first value in the
    ///                  `arguments` array; a `%2` indicates the second argument; and so on. The
    ///                  interpretation of each value depends on the formatting information
    ///                  associated with the insert in the message definition. The default is to
    ///                  treat each value as a pointer to a null-terminated string. By default, the
    ///                  `arguments` parameter is of type `*mut VaList`, which is a language- and
    ///                  implementation-specific data type for describing a variable number of
    ///                  arguments. The state of the [`VaList`] argument is undefined upon return
    ///                  from the function. If you do not have a pointer of type `*mut VaList`,
    ///                  then specify the [`FORMAT_MESSAGE_ARGUMENT_ARRAY`] flag and pass a pointer
    ///                  to an array of [`DWORD_PTR`] values; those values are input to the message
    ///                  formatted as the insert values. Each insert must have a corresponding
    ///                  element in the array.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the number of [`TCHAR`]s stored in the output
    /// buffer, excluding the terminating null character.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    pub fn FormatMessageW(
        flags: DWORD,
        source: LPCVOID,
        message_id: DWORD,
        language_id: DWORD,
        buffer: LPWSTR,
        size: DWORD,
        arguments: *mut VaList,
    ) -> DWORD;
}
