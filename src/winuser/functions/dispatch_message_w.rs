use crate::{LRESULT, MSG};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetMessage, IsDialogMessage, WM_TIMER};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Dispatches a message to a window procedure. It is typically used to dispatch a message
    /// retrieved by the [`GetMessage`] function.
    ///
    /// # Parameters
    ///  * `msg` - A pointer to a structure that contains the message.
    ///
    /// # Return Value
    /// The return value specifies the value returned by the window procedure. Although its meaning
    /// depends on the message being dispatched, the return value generally is ignored.
    ///
    /// # Remarks
    /// The [`MSG`] structure must contain valid message values. If the `msg` parameter points to a
    /// [`WM_TIMER`] message and the `l_param` parameter of the [`WM_TIMER`] message is not
    /// [`null_mut`], `l_param` points to a function that is called instead of the window
    /// procedure.
    ///
    /// Note that the application is responsible for retrieving and dispatching input messages to
    /// the dialog box. Most applications use the main message loop for this. However, to permit
    /// the user to move to and to select controls by using the keyboard, the application must call
    /// [`IsDialogMessage`].
    pub fn DispatchMessageW(msg: *const MSG) -> LRESULT;
}
