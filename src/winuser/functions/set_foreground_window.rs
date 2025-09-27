use crate::{BOOL, HWND};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    AllowSetForegroundWindow, LockSetForegroundWindow, SystemParametersInfo,
    SPI_GETFOREGROUNDLOCKTIMEOUT,
};

#[link(name = "User32")]
extern "system" {
    /// Brings the thread that created the specified window into the foreground and activates the
    /// window. Keyboard input is directed to the window, and various visual cues are changed for
    /// the user. The system assigns a slightly higher priority to the thread that created the
    /// foreground window than it does to other threads.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window that should be activated and brought to the foreground.
    ///
    /// # Return Value
    /// If the window was brought to the foreground, the return value is nonzero.
    ///
    /// If the window was not brought to the foreground, the return value is zero.
    ///
    /// # Remarks
    /// The system restricts which processes can set the foreground window. A process can set the
    /// foreground window by calling [`SetForegroundWindow`] only if:
    ///  - All of the following conditions are true:
    ///    - The calling process belongs to a desktop application, not a UWP app or a Windows Store
    ///      app designed for Windows 8 or 8.1.
    ///    - The foreground process has not disabled calls to [`SetForegroundWindow`] by a previous
    ///      call to the [`LockSetForegroundWindow`] function.
    ///    - The foreground lock time-out has expired (see [`SPI_GETFOREGROUNDLOCKTIMEOUT`] in
    ///      [`SystemParametersInfo`]).
    ///    - No menus are active.
    ///  - Additionally, at least one of the following conditions is true:
    ///    - The calling process is the foreground process.
    ///    - The calling process was started by the foreground process.
    ///    - There is currently no foreground window, and thus no foreground process.
    ///    - The calling process received the last input event.
    ///    - Either the foreground process or the calling process is being debugged.
    ///
    /// It is possible for a process to be denied the right to set the foreground window even if it
    /// meets these conditions.
    ///
    /// An application cannot force a window to the foreground while the user is working with
    /// another window. Instead, Windows flashes the taskbar button of the window to notify the
    /// user.
    ///
    /// A process that can set the foreground window can enable another process to set the
    /// foreground window by calling the [`AllowSetForegroundWindow`] function. The process
    /// specified by the dwProcessId parameter to [`AllowSetForegroundWindow`] loses the ability to
    /// set the foreground window the next time that either the user generates input, unless the
    /// input is directed at that process, or the next time a process calls
    /// [`AllowSetForegroundWindow`], unless the same process is specified as in the previous call
    /// to [`AllowSetForegroundWindow`].
    ///
    /// The foreground process can disable calls to [`SetForegroundWindow`] by calling the
    /// [`LockSetForegroundWindow`] function.
    pub fn SetForegroundWindow(wnd: HWND) -> BOOL;
}
