use crate::{DWORD, HWND, LPDWORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::GetLastError;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Retrieves the identifier of the thread that created the specified window and, optionally,
    /// the identifier of the process that created the window.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window.
    ///  * `process_id` - A pointer to a variable that receives the process identifier. If this
    ///                   parameter is not [`null_mut`], [`GetWindowThreadProcessId`] copies the
    ///                   identifier of the process to the variable; otherwise, it does not. If the
    ///                   function fails, the value of the variable is unchanged.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the identifier of the thread that created the
    /// window. If the window handle is invalid, the return value is zero. To get extended error
    /// information, call [`GetLastError`].
    pub fn GetWindowThreadProcessId(wnd: HWND, process_id: LPDWORD) -> DWORD;
}
