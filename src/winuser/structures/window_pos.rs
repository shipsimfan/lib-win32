use crate::{HWND, UINT};
use std::{ffi::c_int, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SetWindowPos, SWP_DRAWFRAME, SWP_FRAMECHANGED, SWP_HIDEWINDOW, SWP_NOACTIVATE, SWP_NOCOPYBITS,
    SWP_NOMOVE, SWP_NOOWNERZORDER, SWP_NOREDRAW, SWP_NOREPOSITION, SWP_NOSENDCHANGING, SWP_NOSIZE,
    SWP_NOZORDER, SWP_SHOWWINDOW, WM_NCCALCSIZE, WM_WINDOWPOSCHANGING,
};

/// Contains information about the size and position of a window.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WINDOWPOS {
    /// A handle to the window.
    pub wnd: HWND,

    /// The position of the window in Z order (front-to-back position). This member can be a handle
    /// to the window behind which this window is placed, or can be one of the special values
    /// listed with the [`SetWindowPos`] function.
    pub wnd_insert_after: HWND,

    /// The position of the left edge of the window.
    pub x: c_int,

    /// The position of the top edge of the window.
    pub y: c_int,

    /// The window width, in pixels.
    pub cx: c_int,

    /// The window height, in pixels.
    pub cy: c_int,

    /// The window position. This member can be one or more of the following values:
    ///  * [`SWP_DRAWFRAME`] - Draws a frame (defined in the window's class description) around the
    ///                        window. Same as the [`SWP_FRAMECHANGED`] flag.
    ///  * [`SWP_FRAMECHANGED`] - Sends a [`WM_NCCALCSIZE`] message to the window, even if the
    ///                           window's size is not being changed. If this flag is not
    ///                           specified, [`WM_NCCALCSIZE`] is sent only when the window's size
    ///                           is being changed.
    ///  * [`SWP_HIDEWINDOW`] - Hides the window.
    ///  * [`SWP_NOACTIVATE`] - Does not activate the window. If this flag is not set, the window
    ///                         is activated and moved to the top of either the topmost or
    ///                         non-topmost group (depending on the setting of the
    ///                         `wnd_insert_after` member).
    ///  * [`SWP_NOCOPYBITS`] - Discards the entire contents of the client area. If this flag is
    ///                         not specified, the valid contents of the client area are saved and
    ///                         copied back into the client area after the window is sized or
    ///                         repositioned.
    ///  * [`SWP_NOMOVE`] - Retains the current position (ignores the x and y members).
    ///  * [`SWP_NOOWNERZORDER`] - Does not change the owner window's position in the Z order.
    ///  * [`SWP_NOREDRAW`] - Does not redraw changes. If this flag is set, no repainting of any
    ///                       kind occurs. This applies to the client area, the nonclient area
    ///                       (including the title bar and scroll bars), and any part of the parent
    ///                       window uncovered as a result of the window being moved. When this
    ///                       flag is set, the application must explicitly invalidate or redraw any
    ///                       parts of the window and parent window that need redrawing.
    ///  * [`SWP_NOREPOSITION`] - Does not change the owner window's position in the Z order. Same
    ///                           as the [`SWP_NOOWNERZORDER`] flag.
    ///  * [`SWP_NOSENDCHANGING`] - Prevents the window from receiving the [`WM_WINDOWPOSCHANGING`]
    ///                             message.
    ///  * [`SWP_NOSIZE`] - Retains the current size (ignores the `cx` and `cy` members).
    ///  * [`SWP_NOZORDER`] - Retains the current Z order (ignores the `wnd_insert_after` member).
    ///  * [`SWP_SHOWWINDOW`] - Displays the window.    
    pub flags: UINT,
}

impl Default for WINDOWPOS {
    fn default() -> Self {
        WINDOWPOS {
            wnd: null_mut(),
            wnd_insert_after: null_mut(),
            x: 0,
            y: 0,
            cx: 0,
            cy: 0,
            flags: 0,
        }
    }
}
