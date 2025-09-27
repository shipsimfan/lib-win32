use crate::{BOOL, HWND, UINT};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, SetForegroundWindow, SetWindowLong, HWND_BOTTOM, HWND_NOTOPMOST, HWND_TOP,
    HWND_TOPMOST, SWP_ASYNCWINDOWPOS, SWP_DEFERERASE, SWP_DRAWFRAME, SWP_FRAMECHANGED,
    SWP_HIDEWINDOW, SWP_NOACTIVATE, SWP_NOCOPYBITS, SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOREDRAW,
    SWP_NOREPOSITION, SWP_NOSENDCHANGING, SWP_NOSIZE, SWP_NOZORDER, SWP_SHOWWINDOW, WM_NCCALCSIZE,
    WM_SYNCPAINT, WM_WINDOWPOSCHANGING,
};

#[link(name = "User32")]
extern "system" {
    /// Changes the size, position, and Z order of a child, pop-up, or top-level window. These
    /// windows are ordered according to their appearance on the screen. The topmost window
    /// receives the highest rank and is the first window in the Z order.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window.
    ///  * `wnd_insert_after` - A handle to the window to precede the positioned window in the Z
    ///                         order. This parameter must be a window handle or one of the
    ///                         following values:
    ///    * [`HWND_BOTTOM`] - Places the window at the bottom of the Z order. If the `wnd`
    ///                        parameter identifies a topmost window, the window loses its topmost
    ///                        status and is placed at the bottom of all other windows.
    ///    * [`HWND_NOTOPMOST`] - Places the window above all non-topmost windows (that is, behind
    ///                           all topmost windows). This flag has no effect if the window is
    ///                           already a non-topmost window.
    ///    * [`HWND_TOP`] - Places the window at the top of the Z order.
    ///    * [`HWND_TOPMOST`] - Places the window above all non-topmost windows. The window
    ///                         maintains its topmost position even when it is deactivated.
    ///  * `x` - The new position of the left side of the window, in client coordinates.
    ///  * `y` - The new position of the top of the window, in client coordinates.
    ///  * `cx` - The new width of the window, in pixels.
    ///  * `cy` - The new height of the window, in pixels.
    ///  * `flags` - The window sizing and positioning flags. This parameter can be a combination
    ///              of the following values:
    ///    * [`SWP_ASYNCWINDOWPOS`] - If the calling thread and the thread that owns the window are
    ///                               attached to different input queues, the system posts the
    ///                               request to the thread that owns the window. This prevents the
    ///                               calling thread from blocking its execution while other
    ///                               threads process the request.
    ///    * [`SWP_DEFERERASE`] - Prevents generation of the [`WM_SYNCPAINT`] message.
    ///    * [`SWP_DRAWFRAME`] - Draws a frame (defined in the window's class description) around
    ///                          the window.
    ///    * [`SWP_FRAMECHANGED`] - Applies new frame styles set using the [`SetWindowLong`]
    ///                             function. Sends a [`WM_NCCALCSIZE`] message to the window, even
    ///                             if the window's size is not being changed. If this flag is not
    ///                             specified, [`WM_NCCALCSIZE`] is sent only when the window's
    ///                             size is being changed.
    ///    * [`SWP_HIDEWINDOW`] - Hides the window.
    ///    * [`SWP_NOACTIVATE`] - Does not activate the window. If this flag is not set, the window
    ///                           is activated and moved to the top of either the topmost or
    ///                           non-topmost group (depending on the setting of the
    ///                           `wnd_insert_after` parameter).
    ///    * [`SWP_NOCOPYBITS`] - Discards the entire contents of the client area. If this flag is
    ///                           not specified, the valid contents of the client area are saved
    ///                           and copied back into the client area after the window is sized or
    ///                           repositioned.
    ///    * [`SWP_NOMOVE`] - Retains the current position (ignores X and Y parameters).
    ///    * [`SWP_NOOWNERZORDER`] - Does not change the owner window's position in the Z order.
    ///    * [`SWP_NOREDRAW`] - Does not redraw changes. If this flag is set, no repainting of any
    ///                         kind occurs. This applies to the client area, the nonclient area
    ///                         (including the title bar and scroll bars), and any part of the
    ///                         parent window uncovered as a result of the window being moved. When
    ///                         this flag is set, the application must explicitly invalidate or
    ///                         redraw any parts of the window and parent window that need
    ///                         redrawing.
    ///    * [`SWP_NOREPOSITION`] - Same as the [`SWP_NOOWNERZORDER`] flag.
    ///    * [`SWP_NOSENDCHANGING`] - Prevents the window from receiving the
    ///                               [`WM_WINDOWPOSCHANGING`] message.
    ///    * [`SWP_NOSIZE`] - Retains the current size (ignores the cx and cy parameters).
    ///    * [`SWP_NOZORDER`] - Retains the current Z order (ignores the `wnd_insert_after`
    ///                         parameter).
    ///    * [`SWP_SHOWWINDOW`] - Displays the window.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// As part of the Vista re-architecture, all services were moved off the interactive desktop
    /// into Session 0. `wnd` and window manager operations are only effective inside a session and
    /// cross-session attempts to manipulate the `wnd` will fail.
    ///
    /// If you have changed certain window data using [`SetWindowLong`], you must call
    /// [`SetWindowPos`] for the changes to take effect. Use the following combination for `flags`:
    /// `SWP_NOMOVE | SWP_NOSIZE | SWP_NOZORDER | SWP_FRAMECHANGED`.
    ///
    /// A window can be made a topmost window either by setting the `wnd_insert_after` parameter to
    /// [`HWND_TOPMOST`] and ensuring that the [`SWP_NOZORDER`] flag is not set, or by setting a
    /// window's position in the Z order so that it is above any existing topmost windows. When a
    /// non-topmost window is made topmost, its owned windows are also made topmost. Its owners,
    /// however, are not changed.
    ///
    /// If neither the [`SWP_NOACTIVATE`] nor [`SWP_NOZORDER`] flag is specified (that is, when the
    /// application requests that a window be simultaneously activated and its position in the Z
    /// order changed), the value specified in `wnd_insert_after` is used only in the following
    /// circumstances:
    ///  - Neither the [`HWND_TOPMOST`] nor [`HWND_NOTOPMOST`] flag is specified in
    ///    `wnd_insert_after`.
    ///  - The window identified by `wnd` is not the active window.
    ///
    /// An application cannot activate an inactive window without also bringing it to the top of
    /// the Z order. Applications can change an activated window's position in the Z order without
    /// restrictions, or it can activate a window and then move it to the top of the topmost or
    /// non-topmost windows.
    ///
    /// If a topmost window is repositioned to the bottom ([`HWND_BOTTOM`]) of the Z order or after
    /// any non-topmost window, it is no longer topmost. When a topmost window is made non-topmost,
    /// its owners and its owned windows are also made non-topmost windows.
    ///
    /// A non-topmost window can own a topmost window, but the reverse cannot occur. Any window
    /// (for example, a dialog box) owned by a topmost window is itself made a topmost window, to
    /// ensure that all owned windows stay above their owner.
    ///
    /// If an application is not in the foreground, and should be in the foreground, it must call
    /// the [`SetForegroundWindow`] function.
    ///
    /// To use [`SetWindowPos`] to bring a window to the top, the process that owns the window must
    /// have [`SetForegroundWindow`] permission.
    pub fn SetWindowPos(
        wnd: HWND,
        wnd_insert_after: HWND,
        x: c_int,
        y: c_int,
        cx: c_int,
        cy: c_int,
        flags: UINT,
    ) -> BOOL;
}
