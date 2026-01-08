use crate::{BOOL, HWND, LPPOINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::POINT;

#[link(name = "User32")]
extern "system" {
    /// The [`ClientToScreen`] function converts the client-area coordinates of a specified point
    /// to screen coordinates.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window whose client area is used for the conversion.
    ///  * `point` - A pointer to a [`POINT`] structure that contains the client coordinates to be
    ///              converted. The new screen coordinates are copied into this structure if the
    ///              function succeeds.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero.
    ///
    /// # Remarks
    /// The [`ClientToScreen`] function replaces the client-area coordinates in the [`POINT`]
    /// structure with the screen coordinates. The screen coordinates are relative to the
    /// upper-left corner of the screen. Note, a screen-coordinate point that is above the window's
    /// client area has a negative y-coordinate. Similarly, a screen coordinate to the left of a
    /// client area has a negative x-coordinate.
    ///
    /// All coordinates are device coordinates.
    pub fn ClientToScreen(wnd: HWND, point: LPPOINT) -> BOOL;
}
