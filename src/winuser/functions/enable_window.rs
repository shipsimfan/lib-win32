use crate::{BOOL, HWND};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateWindow, CreateWindowEx, FALSE, TRUE, WM_CANCELMODE, WM_ENABLE, WS_DISABLED};

#[link(name = "User32")]
extern "system" {
    /// Enables or disables mouse and keyboard input to the specified window or control. When input
    /// is disabled, the window does not receive input such as mouse clicks and key presses. When
    /// input is enabled, the window receives all input.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window to be enabled or disabled.
    ///  * `enable` - Indicates whether to enable or disable the window. If this parameter is
    ///               [`TRUE`], the window is enabled. If the parameter is [`FALSE`], the window is
    ///               disabled.
    ///
    /// # Return Value
    /// If the window was previously disabled, the return value is nonzero.
    ///
    /// If the window was not previously disabled, the return value is zero.
    ///
    /// # Remarks
    /// If the window is being disabled, the system sends a [`WM_CANCELMODE`] message. If the
    /// enabled state of a window is changing, the system sends a [`WM_ENABLE`] message after the
    /// [`WM_CANCELMODE`] message. (These messages are sent before EnableWindow returns.) If a
    /// window is already disabled, its child windows are implicitly disabled, although they are
    /// not sent a [`WM_ENABLE`] message.
    ///
    /// A window must be enabled before it can be activated. For example, if an application is
    /// displaying a modeless dialog box and has disabled its main window, the application must
    /// enable the main window before destroying the dialog box. Otherwise, another window will
    /// receive the keyboard focus and be activated. If a child window is disabled, it is ignored
    /// when the system tries to determine which window should receive mouse messages.
    ///
    /// By default, a window is enabled when it is created. To create a window that is initially
    /// disabled, an application can specify the [`WS_DISABLED`] style in the [`CreateWindow`] or
    /// [`CreateWindowEx`] function. After a window has been created, an application can use
    /// [`EnableWindow`] to enable or disable the window.
    ///
    /// An application can use this function to enable or disable a control in a dialog box. A
    /// disabled control cannot receive the keyboard focus, nor can a user gain access to it.
    pub fn EnableWindow(wnd: HWND, enable: BOOL) -> BOOL;
}
