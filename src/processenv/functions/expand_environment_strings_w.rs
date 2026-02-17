use crate::{DWORD, LPCWSTR, LPWSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, TCHAR};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Expands environment-variable strings and replaces them with the values defined for the
    /// current user.
    ///
    /// To specify the environment block for a particular user or the system, use the
    /// [`ExpandEnvironmentStringsForUser`] function.
    ///
    /// # Parameters
    ///  * `src` - A buffer that contains one or more environment-variable strings in the form:
    ///            "%variableName%". For each such reference, the "%variableName%" portion is
    ///            replaced with the current value of that environment variable. Case is ignored
    ///            when looking up the environment-variable name. If the name is not found, the
    ///            "%variableName%" portion is left unexpanded. Note that this function does not
    ///            support all the features that Cmd.exe supports. For example, it does not support
    ///            %variableName:str1=str2% or %variableName:~offset,length%.
    ///  * `dst` - A pointer to a buffer that receives the result of expanding the environment
    ///            variable strings in the `src` buffer. Note that this buffer cannot be the same
    ///            as the `src` buffer.
    ///  * `size` - The maximum number of characters that can be stored in the buffer pointed to by
    ///             the `dst` parameter. When using ANSI strings, the buffer size should be the
    ///             string length, plus terminating null character, plus one. When using Unicode
    ///             strings, the buffer size should be the string length plus the terminating null
    ///             character.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the number of [`TCHAR`]s stored in the
    /// destination buffer, including the terminating null character. If the destination buffer is
    /// too small to hold the expanded string, the return value is the required buffer size, in
    /// characters.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// To replace folder names in a fully qualified path with their associated
    /// environment-variable strings, use the [`PathUnExpandEnvStrings`] function.
    ///
    /// To retrieve the list of environment variables for a process, use the
    /// [`GetEnvironmentStrings`] function.
    pub fn ExpandEnvironmentStringsW(src: LPCWSTR, dst: LPWSTR, size: DWORD) -> DWORD;
}
