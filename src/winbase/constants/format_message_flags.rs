use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, GetLastError, LocalFree, ERROR_MORE_DATA, ERROR_RESOURCE_TYPE_NOT_FOUND, LPTSTR,
    TCHAR,
};
#[allow(unused_imports)]
use std::{ffi::VaList, ptr::null};

/// The function allocates a buffer large enough to hold the formatted message, and places a
/// pointer to the allocated buffer at the address specified by `buffer`. The `buffer` parameter is
/// a pointer to an [`LPTSTR`]; you must cast the pointer to an [`LPTSTR`] (for example,
/// `&mut lpBuffer as *mut _ as _`). The `size` parameter specifies the minimum number of
/// [`TCHAR`]s to allocate for an output message buffer. The caller should use the [`LocalFree`]
/// function to free the buffer when it is no longer needed.
///
/// If the length of the formatted message exceeds 128K bytes, then [`FormatMessage`] will fail and
/// a subsequent call to [`GetLastError`] will return [`ERROR_MORE_DATA`].
///
/// In previous versions of Windows, this value was not available for use when compiling Windows
/// Store apps. As of Windows 10 this value can be used.
///
/// **Windows Server 2003 and Windows XP:**
///
/// If the length of the formatted message exceeds 128K bytes, then [`FormatMessage`] will not
/// automatically fail with an error of [`ERROR_MORE_DATA`].
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: DWORD = 0x00000100;

/// Insert sequences in the message definition such as `%1` are to be ignored and passed through to
/// the output buffer unchanged. This flag is useful for fetching a message for later formatting.
/// If this flag is set, the `arguments` parameter is ignored.
pub const FORMAT_MESSAGE_IGNORE_INSERTS: DWORD = 0x00000200;

/// The `source` parameter is a pointer to a null-terminated string that contains a message
/// definition. The message definition may contain insert sequences, just as the message text in a
/// message table resource may. This flag cannot be used with [`FORMAT_MESSAGE_FROM_HMODULE`] or
/// [`FORMAT_MESSAGE_FROM_SYSTEM`].
pub const FORMAT_MESSAGE_FROM_STRING: DWORD = 0x00000400;

/// The `source` parameter is a module handle containing the message-table resource(s) to search.
/// If this `source` handle is [`null`], the current process's application image file will be
/// searched. This flag cannot be used with [`FORMAT_MESSAGE_FROM_STRING`]. If the module has no
/// message table resource, the function fails with [`ERROR_RESOURCE_TYPE_NOT_FOUND`].
pub const FORMAT_MESSAGE_FROM_HMODULE: DWORD = 0x00000800;

/// The function should search the system message-table resource(s) for the requested message. If
/// this flag is specified with [`FORMAT_MESSAGE_FROM_HMODULE`], the function searches the system
/// message table if the message is not found in the module specified by `source`. This flag cannot
/// be used with [`FORMAT_MESSAGE_FROM_STRING`].
///
/// If this flag is specified, an application can pass the result of the [`GetLastError`] function
/// to retrieve the message text for a system-defined error.
pub const FORMAT_MESSAGE_FROM_SYSTEM: DWORD = 0x00001000;

/// The `arguments` parameter is not a [`VaList`] structure, but is a pointer to an array of
/// values that represent the arguments.
///
/// This flag cannot be used with 64-bit integer values. If you are using a 64-bit integer, you
/// must use the [`VaList`] structure.
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: DWORD = 0x00002000;
