use crate::{BOOL, HANDLE, LPDCB};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateFile, GetCommState, GetLastError, DCB};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Configures a communications device according to the specifications in a device-control
    /// block (a [`DCB`] structure). The function reinitializes all hardware and control settings,
    /// but it does not empty output or input queues.
    ///
    /// # Parameters
    ///  * `file` - A handle to the communications device. The [`CreateFile`] function returns this
    ///             handle.
    ///  * `dcb` - A pointer to a [`DCB`] structure that contains the configuration information for
    ///            the specified communications device.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// The [`SetCommState`] function uses a [`DCB`] structure to specify the desired
    /// configuration. The [`GetCommState`] function returns the current configuration.
    ///
    /// To set only a few members of the [`DCB`] structure, you should modify a [`DCB`] structure
    /// that has been filled in by a call to [`GetCommState`]. This ensures that the other members
    /// of the [`DCB`] structure have appropriate values.
    ///
    /// The [`SetCommState`] function fails if the `x_on_char` member of the [`DCB`] structure is
    /// equal to the `x_off_char` member.
    ///
    /// When [`SetCommState`] is used to configure the 8250, the following restrictions apply to
    /// the values for the [`DCB`] structure's `byte_size` and `stop_bits` members:
    ///  - The number of data bits must be 5 to 8 bits.
    pub fn SetCommState(file: HANDLE, dcb: LPDCB) -> BOOL;
}
