use crate::{BOOL, HANDLE, LPCOMMTIMEOUTS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateFile, GetLastError, COMMTIMEOUTS};

#[link(name = "Kernel32")]
extern "system" {
    /// Sets the time-out parameters for all read and write operations on a specified
    /// communications device.
    ///
    /// # Parameters
    ///  * `file` - A handle to the communications device. The [`CreateFile`] function returns this
    ///             handle.
    ///  * `comm_timeouts` - A pointer to a [`COMMTIMEOUTS`] structure that contains the new
    ///                      time-out values.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    pub fn SetCommTimeouts(file: HANDLE, comm_timeouts: LPCOMMTIMEOUTS) -> BOOL;
}
