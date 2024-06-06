use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{WM_DESTROY, WM_QUIT};

#[link(name = "User32")]
extern "system" {
    /// Indicates to the system that a thread has made a request to terminate (quit). It is
    /// typically used in response to a [`WM_DESTROY`] message.
    ///
    /// # Parameters
    ///  * `exit_code` - The application exit code. This value is used as the `w_param` parameter
    ///                  of the [`WM_QUIT`] message.
    ///
    /// # Remarks
    /// The [`PostQuitMessage`] function posts a [`WM_QUIT`] message to the thread's message queue
    /// and returns immediately; the function simply indicates to the system that the thread is
    /// requesting to quit at some time in the future.
    ///
    /// When the thread retrieves the [WM_QUIT] message from its message queue, it should exit its
    /// message loop and return control to the system. The exit value returned to the system must
    /// be the `w_param` parameter of the [`WM_QUIT`] message.
    pub fn PostQuitMessage(exit_code: c_int);
}
