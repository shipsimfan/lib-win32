use crate::{BOOL, HANDLE, LPDCB};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, DCB};

#[link(name = "Kernel32")]
extern "system" {
    /// Retrieves the current control settings for a specified communications device.
    ///
    /// # Parameters
    ///  * `file` - A handle to the communications device. The [`CreateFile`] function returns this
    ///             handle.
    ///  * `dcb` - A pointer to a [`DCB`] structure that receives the control settings information.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    pub fn GetCommState(file: HANDLE, dcb: LPDCB) -> BOOL;
}
