use crate::{BOOL, HWND, LPARAM, UINT, WPARAM};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, GetMessage, PeekMessage, PostMessage, PostQuitMessage, PostThreadMessage,
    HWND_BROADCAST, WM_APPCOMMAND, WM_QUIT, WM_USER,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Places (posts) a message in the message queue associated with the thread that created the
    /// specified window and returns without waiting for the thread to process the message.
    ///
    /// To post a message in the message queue associated with a thread, use the
    /// [`PostThreadMessage`] function.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window whose window procedure is to receive the message. The
    ///            following values have special meanings:
    ///    * [`HWND_BROADCAST`] -  The message is posted to all top-level windows in the system,
    ///                            including disabled or invisible unowned windows, overlapped
    ///                            windows, and pop-up windows. The message is not posted to child
    ///                            windows.
    ///    * [`null_mut`] - The function behaves like a call to [`PostThreadMessage`] with the
    ///                     `thread_id` parameter set to the identifier of the current thread.
    ///  * `msg` - The message to be posted.
    ///  * `w_param` - Additional message-specific information.
    ///  * `l_param` - Additional message-specific information.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// When a message is blocked by UIPI the last error, retrieved with [`GetLastError`], is set
    /// to 5 (access denied).
    ///
    /// Messages in a message queue are retrieved by calls to the [`GetMessage`] or [`PeekMessage`]
    /// function.
    ///
    /// Applications that need to communicate using [`HWND_BROADCAST`] should use the
    /// [`RegisterWindowMessage`] function to obtain a unique message for inter-application
    /// communication.
    ///
    /// The system only does marshalling for system messages (those in the range 0 to
    /// `WM_USER - 1`). To send other messages (those `>= WM_USER`) to another process, you must do
    /// custom marshalling.
    ///
    /// If you send a message in the range below [`WM_USER`] to the asynchronous message functions
    /// ([`PostMessage`], [`SendNotifyMessage`], and [`SendMessageCallback`]), its message
    /// parameters cannot include pointers. Otherwise, the operation will fail. The functions will
    /// return before the receiving thread has had a chance to process the message and the sender
    /// will free the memory before it is used.
    ///
    /// Do not post the [`WM_QUIT`] message using [`PostMessage`]; use the [`PostQuitMessage`]
    /// function.
    ///
    /// An accessibility application can use [`PostMessage`] to post [`WM_APPCOMMAND`] messages to
    /// the shell to launch applications. This functionality is not guaranteed to work for other
    /// types of applications.
    ///
    /// There is a limit of 10,000 posted messages per message queue. This limit should be
    /// sufficiently large. If your application exceeds the limit, it should be redesigned to avoid
    /// consuming so many system resources.
    pub fn PostMessageW(wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> BOOL;
}
