use crate::{DWORD, LPWSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, GetTempPath, TCHAR};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves the path of the directory designated for temporary files.
    ///
    /// # Parameters
    ///  * `buffer_length` - The size of the string buffer identified by `buffer`, in [`TCHAR`]s.
    ///  * `buffer` - A pointer to a string buffer that receives the null-terminated string
    ///               specifying the temporary file path. The returned string ends with a
    ///               backslash, for example, "C:\TEMP\".
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the length, in [`TCHAR`]s, of the string
    /// copied to `buffer`, not including the terminating null character. If the return value is
    /// greater than `buffer_length`, the return value is the length, in [`TCHAR`]s, of the buffer
    /// required to hold the path.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// The maximum possible return value is `MAX_PATH + 1` (261).
    ///
    /// # Remarks
    /// The [`GetTempPath`] function checks for the existence of environment variables in the
    /// following order and uses the first path found:
    ///  1. The path specified by the `TMP` environment variable.
    ///  2. The path specified by the `TEMP` environment variable.
    ///  3. The path specified by the `USERPROFILE` environment variable.
    ///  4. The Windows directory.
    ///
    /// Note that the function does not verify that the path exists, nor does it test to see if the
    /// current process has any kind of access rights to the path. The [`GetTempPath`] function
    /// returns the properly formatted string that specifies the fully qualified path based on the
    /// environment variable search order as previously specified. The application should verify
    /// the existence of the path and adequate access rights to the path prior to any use for file
    /// I/O operations.
    ///
    /// Symbolic link behavior—If the path points to a symbolic link, the temp path name maintains
    /// any symbolic links.
    pub fn GetTempPathW(buffer_length: DWORD, buffer: LPWSTR) -> DWORD;
}
