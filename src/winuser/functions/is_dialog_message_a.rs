use crate::{BOOL, HWND, LPMSG};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    DispatchMessage, IsDialogMessage, TranslateMessage, DM_GETDEFID, DM_SETDEFID, MSG,
    WM_GETDLGCODE, WM_USER,
};

#[link(name = "User32")]
unsafe extern "system" {
    /// Determines whether a message is intended for the specified dialog box and, if it is,
    /// processes the message.
    ///
    /// # Parameters
    ///  * `dlg` - A handle to the dialog box.
    ///  * `msg` - A pointer to an [`MSG`] structure that contains the message to be checked.
    ///
    /// # Return Value
    /// If the message has been processed, the return value is nonzero.
    ///
    /// If the message has not been processed, the return value is zero.
    ///
    /// # Remarks
    /// Although the [`IsDialogMessage`] function is intended for modeless dialog boxes, you can
    /// use it with any window that contains controls, enabling the windows to provide the same
    /// keyboard selection as is used in a dialog box.
    ///
    /// When [`IsDialogMessage`] processes a message, it checks for keyboard messages and converts
    /// them into selections for the corresponding dialog box. For example, the TAB key, when
    /// pressed, selects the next control or group of controls, and the DOWN ARROW key, when
    /// pressed, selects the next control in a group.
    ///
    /// Because the [`IsDialogMessage`] function performs all necessary translating and dispatching
    /// of messages, a message processed by [`IsDialogMessage`] must not be passed to the
    /// [`TranslateMessage`] or [`DispatchMessage`] function.
    ///
    /// IsDialogMessage sends [`WM_GETDLGCODE`] messages to the dialog box procedure to determine
    /// which keys should be processed.
    ///
    /// [`IsDialogMessage`] can send [`DM_GETDEFID`] and [`DM_SETDEFID`] messages to the window.
    /// These messages are defined in the "Winuser.h" header file as [`WM_USER`] and `WM_USER + 1`,
    /// so conflicts are possible with application-defined messages having the same values.
    pub fn IsDialogMessageA(dlg: HWND, msh: LPMSG) -> BOOL;
}
