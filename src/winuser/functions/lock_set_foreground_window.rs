use crate::{BOOL, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, SetForegroundWindow, LSFW_LOCK, LSFW_UNLOCK};

#[link(name = "User32")]
extern "system" {
    /// The foreground process can call the [`LockSetForegroundWindow`] function to disable calls
    /// to the [`SetForegroundWindow`] function.
    ///
    /// # Parameters
    ///  * `lock-code` - Specifies whether to enable or disable calls to [`SetForegroundWindow`].
    ///                  This parameter can be one of the following values:
    ///    * [`LSFW_LOCK`] - Disables calls to [`SetForegroundWindow`].
    ///    * [`LSFW_UNLOCK`] - Enables calls to [`SetForegroundWindow`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// The system automatically enables calls to [`SetForegroundWindow`] if the user presses the
    /// ALT key or takes some action that causes the system itself to change the foreground window
    /// (for example, clicking a background window).
    ///
    /// This function is provided so applications can prevent other applications from making a
    /// foreground change that can interrupt its interaction with the user.
    pub fn LockSetForegroundWindow(lock_code: UINT) -> BOOL;
}
