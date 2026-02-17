use crate::{BOOL, DWORD, LPRECT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    AdjustWindowRectEx, CreateWindow, GetLastError, RECT, WS_HSCROLL, WS_OVERLAPPED, WS_VSCROLL,
};

#[link(name = "User32")]
unsafe extern "system" {
    /// Calculates the required size of the window rectangle, based on the desired client-rectangle
    /// size. The window rectangle can then be passed to the [`CreateWindow`] function to create a
    /// window whose client area is the desired size.
    ///
    /// To specify an extended window style, use the [`AdjustWindowRectEx`] function.
    ///
    /// # Parameters
    ///  * `rect` - A pointer to a [`RECT`] structure that contains the coordinates of the top-left
    ///             and bottom-right corners of the desired client area. When the function returns,
    ///             the structure contains the coordinates of the top-left and bottom-right corners
    ///             of the window to accommodate the desired client area.
    ///  * `style` - The window style of the window whose required size is to be calculated. Note
    ///              that you cannot specify the [`WS_OVERLAPPED`] style.
    ///  * `menu` - Indicates whether the window has a menu.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// A client rectangle is the smallest rectangle that completely encloses a client area. A
    /// window rectangle is the smallest rectangle that completely encloses the window, which
    /// includes the client area and the nonclient area.
    ///
    /// The [`AdjustWindowRect`] function does not add extra space when a menu bar wraps to two or
    /// more rows.
    ///
    /// The [`AdjustWindowRect`] function does not take the [`WS_VSCROLL`] or [`WS_HSCROLL`] styles
    /// into account. To account for the scroll bars, call the [`GetSystemMetrics`] function with
    /// [`SM_CXVSCROLL`] or [`SM_CYHSCROLL`].
    pub fn AdjustWindowRect(rect: LPRECT, style: DWORD, menu: BOOL) -> BOOL;
}
