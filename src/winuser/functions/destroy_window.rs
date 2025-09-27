use crate::{BOOL, HWND};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateDialog, GetLastError, WM_DESTROY, WM_NCDESTROY, WM_PARENTNOTIFY, WS_EX_NOPARENTNOTIFY,
};

#[link(name = "User32")]
extern "system" {
    /// Destroys the specified window. The function sends [`WM_DESTROY`] and [`WM_NCDESTROY`]
    /// messages to the window to deactivate it and remove the keyboard focus from it. The function
    /// also destroys the window's menu, flushes the thread message queue, destroys timers, removes
    /// clipboard ownership, and breaks the clipboard viewer chain (if the window is at the top of
    /// the viewer chain).
    ///
    /// If the specified window is a parent or owner window, [`DestroyWindow`] automatically
    /// destroys the associated child or owned windows when it destroys the parent or owner window.
    /// The function first destroys child or owned windows, and then it destroys the parent or
    /// owner window.
    ///
    /// [`DestroyWindow`] also destroys modeless dialog boxes created by the [`CreateDialog`]
    /// function.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window to be destroyed.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// A thread cannot use [`DestroyWindow`] to destroy a window created by a different thread.
    ///
    /// If the window being destroyed is a child window that does not have the
    /// [`WS_EX_NOPARENTNOTIFY`] style, a [`WM_PARENTNOTIFY`] message is sent to the parent.
    pub fn DestroyWindow(wnd: HWND) -> BOOL;
}
