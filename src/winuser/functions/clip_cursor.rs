use crate::{BOOL, RECT};

// rustdoc imports
#[allow(unused_imports)]
use crate::GetLastError;
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "User32")]
extern "system" {
    /// Confines the cursor to a rectangular area on the screen. If a subsequent cursor position
    /// (set by the [`SetCursorPos`] function or the mouse) lies outside the rectangle, the system
    /// automatically adjusts the position to keep the cursor inside the rectangular area.
    ///
    /// # Parameters
    ///  * `rect` - A pointer to the structure that contains the screen coordinates of the
    ///             upper-left and lower-right corners of the confining rectangle. If this
    ///             parameter is [`null`], the cursor is free to move anywhere on the screen.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// The cursor is a shared resource. If an application confines the cursor, it must release the
    /// cursor by using [`ClipCursor`] before relinquishing control to another application.
    ///
    /// The calling process must have `WINSTA_WRITEATTRIBUTES` access to the window station.
    pub fn ClipCursor(rect: *const RECT) -> BOOL;
}
