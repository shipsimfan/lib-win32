use crate::{BOOL, HWND, LPCSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, SetWindowText, WM_SETTEXT, WS_CAPTION};

#[link(name = "User32")]
extern "system" {
    /// Changes the text of the specified window's title bar (if it has one). If the specified
    /// window is a control, the text of the control is changed. However, [`SetWindowText`] cannot
    /// change the text of a control in another application.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window or control whose text is to be changed.
    ///  * `string` - The new title or control text.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// If the target window is owned by the current process, [`SetWindowText`] causes a
    /// [`WM_SETTEXT`] message to be sent to the specified window or control. If the control is a
    /// list box control created with the [`WS_CAPTION`] style, however, [`SetWindowText`] sets the
    /// text for the control, not for the list box entries.
    ///
    /// To set the text of a control in another process, send the [`WM_SETTEXT`] message directly
    /// instead of calling [`SetWindowText`].
    ///
    /// The [`SetWindowText`] function does not expand tab characters (ASCII code 0x09). Tab
    /// characters are displayed as vertical bar (|) characters.
    pub fn SetWindowTextA(wnd: HWND, string: LPCSTR) -> BOOL;
}
