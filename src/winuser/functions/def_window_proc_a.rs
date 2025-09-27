use crate::{HWND, LPARAM, LRESULT, UINT, WPARAM};

// rustdoc imports
#[allow(unused_imports)]
use crate::DefWindowProc;

#[link(name = "User32")]
extern "system" {
    /// Calls the default window procedure to provide default processing for any window messages
    /// that an application does not process. This function ensures that every message is
    /// processed. [`DefWindowProc`] is called with the same parameters received by the window
    /// procedure.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window procedure that received the message.
    ///  * `msg` - The message.
    ///  * `w_param` - Additional message information. The content of this parameter depends on the
    ///                value of the `msg` parameter.
    ///  * `l_param` - Additional message information. The content of this parameter depends on the
    ///                value of the `msg` parameter.
    ///
    /// # Return Value
    /// The return value is the result of the message processing and depends on the message.
    pub fn DefWindowProcA(wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT;
}
