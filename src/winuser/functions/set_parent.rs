use crate::HWND;

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, HWND_MESSAGE, WM_CHANGEUISTATE, WM_UPDATEUISTATE, WS_CHILD, WS_POPUP};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Changes the parent window of the specified child window.
    ///
    /// # Parameters
    ///  * `wnd_child` - A handle to the child window.
    ///  * `wnd_new_parent` - A handle to the new parent window. If this parameter is [`null_mut`],
    ///                       the desktop window becomes the new parent window. If this parameter
    ///                       is [`HWND_MESSAGE`], the child window becomes a message-only window.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the previous parent window.
    ///
    /// If the function fails, the return value is NULL. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// An application can use the [`SetParent`] function to set the parent window of a pop-up,
    /// overlapped, or child window.
    ///
    /// If the window identified by the `wnd_child` parameter is visible, the system performs the
    /// appropriate redrawing and repainting.
    ///
    /// For compatibility reasons, [`SetParent`] does not modify the [`WS_CHILD`] or [`WS_POPUP`]
    /// window styles of the window whose parent is being changed. Therefore, if `wnd_new_parent`
    /// is [`null_mut`], you should also clear the [`WS_CHILD`] bit and set the [`WS_POPUP`] style
    /// after calling [`SetParent`]. Conversely, if `wnd_new_parent` is not [`null_mut`] and the
    /// window was previously a child of the desktop, you should clear the [`WS_POPUP`] style and
    /// set the [`WS_CHILD`] style before calling [`SetParent`].
    ///
    /// When you change the parent of a window, you should synchronize the UISTATE of both windows.
    /// For more information, see [`WM_CHANGEUISTATE`] and [`WM_UPDATEUISTATE`].
    pub fn SetParent(wnd_child: HWND, wnd_new_parent: HWND) -> HWND;
}
