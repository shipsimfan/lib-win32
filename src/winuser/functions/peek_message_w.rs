use crate::{BOOL, HWND, LPMSG, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    DispatchMessage, PeekMessage, PostMessage, PostThreadMessage, MSG, PM_NOREMOVE, PM_NOYIELD,
    PM_REMOVE, WM_KEYFIRST, WM_MOUSEFIRST, WM_PAINT, WM_QUIT, WM_TIMER,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
extern "system" {
    /// Dispatches incoming nonqueued messages, checks the thread message queue for a posted
    /// message, and retrieves the message (if any exist).
    ///
    /// # Parameters
    ///  * `msg` - A pointer to an [`MSG`] structure that receives message information.
    ///  * `wnd` - A handle to the window whose messages are to be retrieved. The window must
    ///            belong to the current thread. If `wnd` is [`null_mut`], [`PeekMessage`]
    ///            retrieves messages for any window that belongs to the current thread, and any
    ///            messages on the current thread's message queue whose hwnd value is [`null_mut`]
    ///            (see the [`MSG`] structure). Therefore if `wnd` is [`null_mut`], both window
    ///            messages and thread messages are processed. If `wnd` is `-1`, [`PeekMessage`]
    ///            retrieves only messages on the current thread's message queue whose `wnd` value
    ///            is [`null_mut`], that is, thread messages as posted by [`PostMessage`] (when the
    ///            `wnd` parameter is [`null_mut`]) or [`PostThreadMessage`].
    ///  * `msg_filter_min` - The value of the first message in the range of messages to be
    ///                       examined. Use [`WM_KEYFIRST`] to specify the first keyboard message
    ///                       or [`WM_MOUSEFIRST`] to specify the first mouse message. If
    ///                       `msg_filter_min` and `msg_filter_max` are both zero, [`PeekMessage`]
    ///                       returns all available messages (that is, no range filtering is
    ///                       performed).
    ///  * `msg_filter_max` - The value of the last message in the range of messages to be
    ///                       examined. If `msg_filter_min` and `msg_filter_max` are both zero,
    ///                       [`PeekMessage`] returns all available messages (that is, no range
    ///                       filtering is performed).
    ///  * `remove_msg` - Specifies how messages are to be handled. This parameter can be one or
    ///                   more of the following values:
    ///    * [`PM_NOREMOVE`] - Messages are not removed from the queue after processing by
    ///                        [`PeekMessage`].
    ///    * [`PM_REMOVE`] - Messages are removed from the queue after processing by
    ///                      [`PeekMessage`].
    ///    * [`PM_NOYIELD`] - Prevents the system from releasing any thread that is waiting for the
    ///                       caller to go idle (see [`WaitForInputIdle`]). Combine this value with
    ///                       either [`PM_NOREMOVE`] or [`PM_REMOVE`].
    ///
    /// # Return Value
    /// If a message is available, the return value is nonzero.
    ///
    /// If no messages are available, the return value is zero.
    ///
    /// # Remarks
    /// [`PeekMessage`] retrieves messages associated with the window identified by the `wnd`
    /// parameter or any of its children as specified by the [`IsChild`] function, and within the
    /// range of message values given by the `msg_filter_min` and `msg_filter_max` parameters. Note
    /// that an application can only use the low word in the `msg_filter_min` and `msg_filter_max`
    /// parameters; the high word is reserved for the system.
    ///
    /// Note that [`PeekMessage`] always retrieves [`WM_QUIT`] messages, no matter which values you
    /// specify for `msg_filter_min` and `msg_filter_max`.
    ///
    /// During this call, the system dispatches ([`DispatchMessage`]) pending, nonqueued messages,
    /// that is, messages sent to windows owned by the calling thread using the [`SendMessage`],
    /// [`SendMessageCallback`], [`SendMessageTimeout`], or [`SendNotifyMessage`] function. Then
    /// the first queued message that matches the specified filter is retrieved. The system may
    /// also process internal events. If no filter is specified, messages are processed in the
    /// following order:
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
    /// The [`PeekMessage`] function normally does not remove [`WM_PAINT`] messages from the queue.
    /// [`WM_PAINT`] messages remain in the queue until they are processed. However, if a
    /// [`WM_PAINT`] message has a [`null_mut`] update region, [`PeekMessage`] does remove it from
    /// the queue.
    ///
    /// If a top-level window stops responding to messages for more than several seconds, the
    /// system considers the window to be not responding and replaces it with a ghost window that
    /// has the same z-order, location, size, and visual attributes. This allows the user to move
    /// it, resize it, or even close the application. However, these are the only actions available
    /// because the application is actually not responding. When an application is being debugged,
    /// the system does not generate a ghost window.
    pub fn PeekMessageW(
        msg: LPMSG,
        wnd: HWND,
        msg_filter_min: UINT,
        msg_filter_max: UINT,
        remove_msg: UINT,
    ) -> BOOL;
}
