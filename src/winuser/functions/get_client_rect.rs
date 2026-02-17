use crate::{BOOL, HWND, LPRECT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, RECT};

#[link(name = "User32")]
unsafe extern "system" {
    /// Retrieves the coordinates of a window's client area. The client coordinates specify the
    /// upper-left and lower-right corners of the client area. Because client coordinates are
    /// relative to the upper-left corner of a window's client area, the coordinates of the
    /// upper-left corner are `(0,0)`.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window whose client coordinates are to be retrieved.
    ///  * `rect` - A pointer to a [`RECT`] structure that receives the client coordinates. The
    ///             left and top members are zero. The right and bottom members contain the width
    ///             and height of the window.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// In conformance with conventions for the [`RECT`] structure, the bottom-right coordinates of
    /// the returned rectangle are exclusive. In other words, the pixel at `(right, bottom)` lies
    /// immediately outside the rectangle.
    pub fn GetClientRect(wnd: HWND, rect: LPRECT) -> BOOL;
}
