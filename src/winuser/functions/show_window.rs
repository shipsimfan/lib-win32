use crate::{BOOL, HWND};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SW_FORCEMINIMIZE, SW_HIDE, SW_MAXIMIZE, SW_MINIMIZE, SW_NORMAL, SW_RESTORE, SW_SHOW,
    SW_SHOWDEFAULT, SW_SHOWMAXIMIZED, SW_SHOWMINIMIZED, SW_SHOWMINNOACTIVE, SW_SHOWNA,
    SW_SHOWNOACTIVATE, SW_SHOWNORMAL, WS_VISIBLE,
};

#[link(name = "User32")]
extern "system" {
    /// Sets the specified window's show state.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window.
    ///  * `cmd_show` - Controls how the window is to be shown. This parameter is ignored the first
    ///                 time an application calls [`ShowWindow`], if the program that launched the
    ///                 application provides a [`STARTUPINFO`] structure. Otherwise, the first time
    ///                 [`ShowWindow`] is called, the value should be the value obtained by the
    ///                 `WinMain` function in its `cmd_show` parameter. In subsequent calls, this
    ///                 parameter can be one of the following values:
    ///    * [`SW_HIDE`] - Hides the window and activates another window.
    ///    * [`SW_SHOWNORMAL`] or [`SW_NORMAL`] - Activates and displays a window. If the window is
    ///                                           minimized, maximized, or arranged, the system
    ///                                           restores it to its original size and position. An
    ///                                           application should specify this flag when
    ///                                           displaying the window for the first time.
    ///    * [`SW_SHOWMINIMIZED`] - Activates the window and displays it as a minimized window.
    ///    * [`SW_SHOWMAXIMIZED`] or [`SW_MAXIMIZE`] - Activates the window and displays it as a
    ///                                                maximized window.
    ///    * [`SW_SHOWNOACTIVATE`] - Displays a window in its most recent size and position. This
    ///                              value is similar to [`SW_SHOWNORMAL`], except that the window
    ///                              is not activated.
    ///    * [`SW_SHOW`] - Activates the window and displays it in its current size and position.
    ///    * [`SW_MINIMIZE`] - Minimizes the specified window and activates the next top-level
    ///                        window in the Z order.
    ///    * [`SW_SHOWMINNOACTIVE`] - Displays the window as a minimized window. This value is
    ///                               similar to [`SW_SHOWMINIMIZED`], except the window is not
    ///                               activated.
    ///    * [`SW_SHOWNA`] - Displays the window in its current size and position. This value is
    ///                      similar to [`SW_SHOW`], except that the window is not activated.
    ///    * [`SW_RESTORE`] - Activates and displays the window. If the window is minimized,
    ///                       maximized, or arranged, the system restores it to its original size
    ///                       and position. An application should specify this flag when restoring
    ///                       a minimized window.
    ///    * [`SW_SHOWDEFAULT`] - Sets the show state based on the `SW_` value specified in the
    ///                           [`STARTUPINFO`] structure passed to the [`CreateProcess`]
    ///                           function by the program that started the application.
    ///    * [`SW_FORCEMINIMIZE`] - Minimizes a window, even if the thread that owns the window is
    ///                             not responding. This flag should only be used when minimizing
    ///                             windows from a different thread.
    ///
    /// # Return Value
    /// If the window was previously visible, the return value is nonzero.
    ///
    /// If the window was previously hidden, the return value is zero.
    ///
    /// # Remarks
    /// To perform certain special effects when showing or hiding a window, use [`AnimateWindow`].
    ///
    /// The first time an application calls [`ShowWindow`], it should use the `WinMain` function's
    /// `cmd_show` parameter as its `cmd_show` parameter. Subsequent calls to [`ShowWindow`] must
    /// use one of the values in the given list, instead of the one specified by the `WinMain`
    /// function's `cmd_show` parameter.
    ///
    /// As noted in the discussion of the `cmd_show` parameter, the `cmd_show` value is ignored in
    /// the first call to [`ShowWindow`] if the program that launched the application specifies
    /// startup information in the structure. In this case, [`ShowWindow`] uses the information
    /// specified in the [`STARTUPINFO`] structure to show the window. On subsequent calls, the
    /// application must call [`ShowWindow`] with `cmd_show` set to [`SW_SHOWDEFAULT`] to use the
    /// startup information provided by the program that launched the application. This behavior is
    /// designed for the following situations:
    ///  - Applications create their main window by calling [`CreateWindow`] with the
    ///    [`WS_VISIBLE`] flag set.
    ///  - Applications create their main window by calling [`CreateWindow`] with the
    ///    [`WS_VISIBLE`] flag cleared, and later call [`ShowWindow`] with the [`SW_SHOW`] flag set
    ///    to make it visible.
    pub fn ShowWindow(wnd: HWND, cmd_show: c_int) -> BOOL;
}
