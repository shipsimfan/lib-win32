use crate::HWND;

/// Places the window at the bottom of the Z order. If the `wnd` status and is placed at the bottom
/// of all other windows.
pub const HWND_BOTTOM: HWND = 1 as HWND;

/// Places the window above all non-topmost windows (that is, behind already a non-topmost window.
pub const HWND_NOTOPMOST: HWND = -2isize as HWND;

/// Places the window at the top of the Z order.
pub const HWND_TOP: HWND = 0 as HWND;

/// Places the window above all non-topmost windows. The window maintains its topmost position even
/// when it is deactivated.
pub const HWND_TOPMOST: HWND = -1isize as HWND;
