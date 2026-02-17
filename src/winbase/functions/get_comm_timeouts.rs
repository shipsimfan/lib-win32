use crate::{BOOL, HANDLE, LPCOMMTIMEOUTS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateFile, GetLastError, SetCommTimeouts, COMMTIMEOUTS};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves the time-out parameters for all read and write operations on a specified
    /// communications device.
    ///
    /// # Parameters
    ///  * `file` - A handle to the communications device. The [`CreateFile`] function returns this
    ///             handle.
    ///  * `comm_timeouts` - A pointer to a [`COMMTIMEOUTS`] structure in which the time-out
    ///                      information is returned.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// For more information about time-out values for communications devices, see the
    /// [`SetCommTimeouts`] function.
    pub fn GetCommTimeouts(file: HANDLE, comm_timeouts: LPCOMMTIMEOUTS) -> BOOL;
}
