use crate::{DWORD, HWND, LPARAM, POINT, UINT, WPARAM};
use std::ptr::null_mut;

/// Contains message information from a thread's message queue.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct MSG {
    /// A handle to the window whose window procedure receives the message. This member is
    /// [`null_mut`] when the message is a thread message.
    pub wnd: HWND,

    /// The message identifier. Applications can only use the low word; the high word is reserved
    /// by the system.
    pub message: UINT,

    /// Additional information about the message. The exact meaning depends on the value of the
    /// message member.
    pub w_param: WPARAM,

    /// Additional information about the message. The exact meaning depends on the value of the
    /// message member.
    pub l_param: LPARAM,

    /// The time at which the message was posted.
    pub time: DWORD,

    /// The cursor position, in screen coordinates, when the message was posted.
    pub pt: POINT,

    #[allow(missing_docs)]
    pub private: DWORD,
}

impl Default for MSG {
    fn default() -> Self {
        MSG {
            wnd: null_mut(),
            message: 0,
            w_param: 0,
            l_param: 0,
            time: 0,
            pt: POINT::default(),
            private: 0,
        }
    }
}
