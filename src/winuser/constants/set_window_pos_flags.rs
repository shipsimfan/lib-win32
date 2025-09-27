use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SetWindowLong, WM_SYNCPAINT, WM_WINDOWPOSCHANGING};

/// If the calling thread and the thread that owns the window are threads process the request.
pub const SWP_ASYNCWINDOWPOS: UINT = 0x4000;

/// Prevents generation of the [`WM_SYNCPAINT`] message.
pub const SWP_DEFERERASE: UINT = 0x2000;

/// Draws a frame (defined in the window's class description) around the window.
pub const SWP_DRAWFRAME: UINT = 0x0020;

/// Applies new frame styles set using the [`SetWindowLong`] size is being changed.
pub const SWP_FRAMECHANGED: UINT = 0x0020;

/// Hides the window.
pub const SWP_HIDEWINDOW: UINT = 0x0080;

/// Does not activate the window. If this flag is not set, the window `wnd_insert_after`
/// parameter).
pub const SWP_NOACTIVATE: UINT = 0x0010;

/// Discards the entire contents of the client area. If this flag is repositioned.
pub const SWP_NOCOPYBITS: UINT = 0x0100;

/// Retains the current position (ignores X and Y parameters).
pub const SWP_NOMOVE: UINT = 0x0002;

/// Does not change the owner window's position in the Z order.
pub const SWP_NOOWNERZORDER: UINT = 0x0200;

/// Does not redraw changes. If this flag is set, no repainting of any redrawing.
pub const SWP_NOREDRAW: UINT = 0x0008;

/// Same as the [`SWP_NOOWNERZORDER`] flag.
pub const SWP_NOREPOSITION: UINT = 0x0200;

/// Prevents the window from receiving the [`WM_WINDOWPOSCHANGING`] message.
pub const SWP_NOSENDCHANGING: UINT = 0x0400;

/// Retains the current size (ignores the cx and cy parameters).
pub const SWP_NOSIZE: UINT = 0x0001;

/// Retains the current Z order (ignores the `wnd_insert_after` parameter).
pub const SWP_NOZORDER: UINT = 0x0004;

/// Displays the window.
pub const SWP_SHOWWINDOW: UINT = 0x0040;
