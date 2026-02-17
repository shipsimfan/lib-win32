use crate::{BOOL, HWND, LPMSG, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, PeekMessage, PostMessage, PostThreadMessage, MSG, WM_INPUT, WM_KEYFIRST,
    WM_KEYLAST, WM_MOUSEFIRST, WM_MOUSELAST, WM_PAINT, WM_QUIT, WM_TIMER,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Retrieves a message from the calling thread's message queue. The function dispatches
    /// incoming sent messages until a posted message is available for retrieval.
    ///
    /// [`GetMessage`] functions like [`PeekMessage`], however, [`GetMessage`] blocks until a
    /// message is posted before returning.
    ///
    /// # Parameters
    ///  * `msg` - A pointer to an [`MSG`] structure that receives message information from the
    ///            thread's message queue.
    ///  * `wnd` - A handle to the window whose messages are to be retrieved. The window must
    ///            belong to the current thread. If `wnd` is [`null_mut`], [`GetMessage`] retrieves
    ///            messages for any window that belongs to the current thread, and any messages on
    ///            the current thread's message queue whose `wnd` value is [`null_mut`] (see the
    ///            [`MSG`] structure). Therefore if `wnd` is [`null_mut`], both window messages and
    ///            thread messages are processed. If `wnd` is -1, [`GetMessage`] retrieves only
    ///            messages on the current thread's message queue whose `wnd` value is
    ///            [`null_mut`], that is, thread messages as posted by [`PostMessage`] (when the
    ///            `wnd` parameter is [`null_mut`]) or [`PostThreadMessage`].
    ///  * `msg_filter_min` - The integer value of the lowest message value to be retrieved. Use
    ///                       [`WM_KEYFIRST`] to specify the first keyboard message or
    ///                       [`WM_MOUSEFIRST`] to specify the first mouse message. Use
    ///                       [`WM_INPUT`] here and in `msg_filter_max` to specify only the
    ///                       [`WM_INPUT`] messages. If `msg_filter_min` and `msg_filter_max` are
    ///                       both zero, [`GetMessage`] returns all available messages (that is, no
    ///                       range filtering is performed).
    ///  * `msg_filter_max` - The integer value of the highest message value to be retrieved. Use
    ///                       [`WM_KEYLAST`] to specify the last keyboard message or
    ///                       [`WM_MOUSELAST`] to specify the last mouse message. Use [`WM_INPUT`]
    ///                       here and in `msg_filter_min` to specify only the [`WM_INPUT`]
    ///                       messages. If `msg_filter_min` and `msg_filter_max` are both zero,
    ///                       [`GetMessage`] returns all available messages (that is, no range
    ///                       filtering is performed).
    ///
    /// # Return Value
    /// If the function retrieves a message other than [`WM_QUIT`], the return value is nonzero.
    ///
    /// If the function retrieves the [`WM_QUIT`] message, the return value is zero.
    ///
    /// If there is an error, the return value is -1. For example, the function fails if `wnd` is
    /// an invalid window handle or `msg` is an invalid pointer. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// An application typically uses the return value to determine whether to end the main message
    /// loop and exit the program.
    ///
    /// The [`GetMessage`] function retrieves messages associated with the window identified by the
    /// `wnd` parameter or any of its children, as specified by the [`IsChild`] function, and
    /// within the range of message values given by the `msg_filter_min` and `msg_filter_max`
    /// parameters. Note that an application can only use the low word in the `msg_filter_min` and
    /// `msg_filter_max` parameters; the high word is reserved for the system.
    ///
    /// Note that [`GetMessage`] always retrieves [`WM_QUIT`] messages, no matter which values you
    /// specify for `msg_filter_min` and `msg_filter_max`.
    ///
    /// During this call, the system delivers pending, nonqueued messages, that is, messages sent
    /// to windows owned by the calling thread using the [`SendMessage`], [`SendMessageCallback`],
    /// [`SendMessageTimeout`], or [`SendNotifyMessage`] function. Then the first queued message
    /// that matches the specified filter is retrieved. The system may also process internal
    /// events. If no filter is specified, messages are processed in the following order:
    ///  - Sent messages
    ///  - Posted messages
    ///  - Input (hardware) messages and system internal events
    ///  - Sent messages (again)
    ///  - [`WM_PAINT`] messages
    ///  - [`WM_TIMER`] messages
    ///
    /// To retrieve input messages before posted messages, use the `msg_filter_min` and
    /// `msg_filter_max` parameters.
    ///
    /// [`GetMessage`] does not remove [`WM_PAINT`] messages from the queue. The messages remain in
    /// the queue until processed.
    ///
    /// If a top-level window stops responding to messages for more than several seconds, the
    /// system considers the window to be not responding and replaces it with a ghost window that
    /// has the same z-order, location, size, and visual attributes. This allows the user to move
    /// it, resize it, or even close the application. However, these are the only actions available
    /// because the application is actually not responding. When in the debugger mode, the system
    /// does not generate a ghost window.
    pub fn GetMessageW(msg: LPMSG, wnd: HWND, msg_filter_min: UINT, msg_filter_max: UINT) -> BOOL;
}
