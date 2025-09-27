use crate::{HWND, LPARAM, LRESULT, UINT, WNDPROC, WPARAM};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CallWindowProc, GetWindowLong, SetWindowLong, DWL_DLGPROC, GWL_WNDPROC};

#[link(name = "User32")]
extern "system" {
    /// Passes message information to the specified window procedure.
    ///
    /// # Parameters
    ///  * `prev_wnd_func` - The previous window procedure. If this value is obtained by calling
    ///                      the [`GetWindowLong`] function with the `index` parameter set to
    ///                      [`GWL_WNDPROC`] or [`DWL_DLGPROC`], it is actually either the address
    ///                      of a window or dialog box procedure, or a special internal value
    ///                      meaningful only to [`CallWindowProc`].
    ///  * `wnd` - A handle to the window procedure to receive the message.
    ///  * `msg` - The message.
    ///  * `w_param` - Additional message-specific information. The contents of this parameter
    ///                depend on the value of the `msg` parameter.
    ///  * `l_param` - Additional message-specific information. The contents of this parameter
    ///                depend on the value of the `msg` parameter.
    ///
    /// # Return Value
    /// The return value specifies the result of the message processing and depends on the message
    /// sent.
    ///
    /// # Remarks
    /// Use the [`CallWindowProc`] function for window subclassing. Usually, all windows with the
    /// same class share one window procedure. A subclass is a window or set of windows with the
    /// same class whose messages are intercepted and processed by another window procedure (or
    /// procedures) before being passed to the window procedure of the class.
    ///
    /// The [`SetWindowLong`] function creates the subclass by changing the window procedure
    /// associated with a particular window, causing the system to call the new window procedure
    /// instead of the previous one. An application must pass any messages not processed by the new
    /// window procedure to the previous window procedure by calling [`CallWindowProc`]. This
    /// allows the application to create a chain of window procedures.
    pub fn CallWindowProcA(
        prev_wnd_func: WNDPROC,
        wnd: HWND,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT;
}
