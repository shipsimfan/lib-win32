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
    /// Formats a message string.
    ///
    /// The function requires a message definition as input. The message
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
    ///
    /// # Remarks
    /// Within the message text, several escape sequences are supported for dynamically formatting
    /// the message. These escape sequences and their meanings are shown in the following lists.
    /// All escape sequences start with the percent character (%).
    ///  * `%0` - Terminates a message text line without a trailing new line character. This escape
    ///           sequence can be used to build up long lines or to terminate the message itself
    ///           without a trailing new line character. It is useful for prompt messages.
    ///  * `%n!format string!` - Identifies an insert sequence. The value of n can be in the range
    ///                          from 1 through 99. The format string (which must be surrounded by
    ///                          exclamation marks) is optional and defaults to `!s!` if not
    ///                          specified. For more information, see Format Specification Fields.
    ///                          The format string can include a width and precision specifier for
    ///                          strings and a width specifier for integers. Use an asterisk `*` to
    ///                          specify the width and precision. For example, `%1!.*s!` or
    ///                          `%1!*u!`. If you do not use the width and precision specifiers,
    ///                          the insert numbers correspond directly to the input arguments. For
    ///                          example, if the source string is "%1 %2 %1" and the input
    ///                          arguments are "Bill" and "Bob", the formatted output string is
    ///                          "Bill Bob Bill". However, if you use a width and precision
    ///                          specifier, the insert numbers do not correspond directly to the
    ///                          input arguments. For example, the insert numbers for the previous
    ///                          example could change to "%1!*.*s! %4 %5!*s!". The insert numbers
    ///                          depend on whether you use an arguments array
    ///                          ([`FORMAT_MESSAGE_ARGUMENT_ARRAY`]) or a [`VaList`]. For an
    ///                          arguments array, the next insert number is `n+2` if the previous
    ///                          format string contained one asterisk and is `n+3` if two asterisks
    ///                          were specified. For a [`VaList`], the next insert number is `n+1`
    ///                          if the previous format string contained one asterisk and is `n+2`
    ///                          if two asterisks were specified. If you want to repeat "Bill", as
    ///                          in the previous example, the arguments must include "Bill" twice.
    ///                          For example, if the source string is "%1!*.*s! %4 %5!*s!", the
    ///                          arguments could be, 4, 2, Bill, Bob, 6, Bill (if using the
    ///                          [`FORMAT_MESSAGE_ARGUMENT_ARRAY`] flag). The formatted string
    ///                          would then be "  Bi Bob   Bill". Repeating insert numbers when the
    ///                          source string contains width and precision specifiers may not
    ///                          yield the intended results. If you replaced `%5` with `%1`, the
    ///                          function would try to print a string at address 6 (likely
    ///                          resulting in an access violation). Floating-point format
    ///                          specifiers—e, E, f, and g—are not supported. The workaround is to
    ///                          use the [`StringCchPrintf`] function to format the floating-point
    ///                          number into a temporary buffer, then use that buffer as the insert
    ///                          string. Inserts that use the `I64` prefix are treated as two
    ///                          32-bit arguments. They must be used before subsequent arguments
    ///                          are used. Note that it may be easier for you to use
    ///                          [`StringCchPrintf`] instead of this prefix.
    ///
    /// Any other nondigit character following a percent character is formatted in the output
    /// message without the percent character. Following are some examples:
    ///  * `%%` - A single percent sign.
    ///  * `%space` - A single space. This format string can be used to ensure the appropriate
    ///               number of trailing spaces in a message text line.
    ///  * `%.` - A single period. This format string can be used to include a single period at the
    ///           beginning of a line without terminating the message text definition.
    ///  * `%!` - A single exclamation point. This format string can be used to include an
    ///           exclamation point immediately after an insert without its being mistaken for the
    ///           beginning of a format string.
    ///  * `%n` - A hard line break when the format string occurs at the end of a line. This format
    ///           string is useful when [`FormatMessage`] is supplying regular line breaks so the
    ///           message fits in a certain width.
    ///  * `%r` - A hard carriage return without a trailing newline character.
    ///  * `%t` - A single tab.
    ///
    /// # Security Remarks
    /// If this function is called without [`FORMAT_MESSAGE_IGNORE_INSERTS`], the `arguments`
    /// parameter must contain enough parameters to satisfy all insertion sequences in the message
    /// string, and they must be of the correct type. Therefore, do not use untrusted or unknown
    /// message strings with inserts enabled because they can contain more insertion sequences than
    /// `arguments` provides, or those that may be of the wrong type. In particular, it is unsafe
    /// to take an arbitrary system error code returned from an API and use
    /// [`FORMAT_MESSAGE_FROM_SYSTEM`] without [`FORMAT_MESSAGE_IGNORE_INSERTS`].
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
