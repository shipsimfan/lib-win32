use crate::{BOOL, HWND};

#[link(name = "User32")]
unsafe extern "system" {
    /// Determines whether the specified window is minimized (iconic).
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window to be tested.
    ///
    /// # Return Value
    /// If the window is iconic, the return value is nonzero.
    ///
    /// If the window is not iconic, the return value is zero.
    pub fn IsIconic(wnd: HWND) -> BOOL;
}
