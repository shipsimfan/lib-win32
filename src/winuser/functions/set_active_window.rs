use crate::HWND;

// rustdoc imports
#[allow(unused_imports)]
use crate::GetLastError;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Activates a window. The window must be attached to the calling thread's message queue.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the top-level window to be activated.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the handle to the window that was previously
    /// active.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// The [`SetActiveWindow`] function activates a window, but not if the application is in the
    /// background. The window will be brought into the foreground (top of Z-Order) if its
    /// application is in the foreground when the system activates the window.
    ///
    /// If the window identified by the `wnd` parameter was created by the calling thread, the
    /// active window status of the calling thread is set to `wnd`. Otherwise, the active window
    /// status of the calling thread is set to [`null_mut`].
    pub fn SetActiveWindow(wnd: HWND) -> HWND;
}
