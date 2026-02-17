use crate::{BOOL, DWORD, LPARAM, UINT, WPARAM};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    DialogBox, DispatchMessage, GetLastError, GetMessage, MessageBox, PeekMessage,
    PostThreadMessage, WaitForSingleObject, ERROR_INVALID_THREAD_ID, ERROR_NOT_ENOUGH_QUOTA, MSG,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "User32")]
unsafe extern "system" {
    /// Posts a message to the message queue of the specified thread. It returns without waiting
    /// for the thread to process the message.
    ///
    /// # Parameters
    ///  * `thread_id` - The identifier of the thread to which the message is to be posted. The
    ///                  function fails if the specified thread does not have a message queue. The
    ///                  system creates a thread's message queue when the thread makes its first
    ///                  call to one of the User or GDI functions. Message posting is subject to
    ///                  UIPI. The thread of a process can post messages only to posted-message
    ///                  queues of threads in processes of lesser or equal integrity level. This
    ///                  thread must have the `SE_TCB_NAME` privilege to post a message to a thread
    ///                  that belongs to a process with the same locally unique identifier (LUID)
    ///                  but is in a different desktop. Otherwise, the function fails and returns
    ///                  [`ERROR_INVALID_THREAD_ID`]. This thread must either belong to the same
    ///                  desktop as the calling thread or to a process with the same LUID.
    ///                  Otherwise, the function fails and returns [`ERROR_INVALID_THREAD_ID`].
    ///  * `msg` - The type of message to be posted.
    ///  * `w_param` - Additional message-specific information.
    ///  * `l_param` - Additional message-specific information.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`]. [`GetLastError`] returns [`ERROR_INVALID_THREAD_ID`] if `thread_id` is
    /// not a valid thread identifier, or if the thread specified by `thread_id` does not have a
    /// message queue. [`GetLastError`] returns [`ERROR_NOT_ENOUGH_QUOTA`] when the message limit
    /// is hit.
    ///
    /// # Remarks
    /// When a message is blocked by UIPI the last error, retrieved with [`GetLastError`], is set
    /// to 5 (access denied).
    ///
    /// The thread to which the message is posted must have created a message queue, or else the
    /// call to [`PostThreadMessage`] fails. Use the following method to handle this situation.
    ///  - Create an event object, then create the thread.
    ///  - Use the [`WaitForSingleObject`] function to wait for the event to be set to the signaled
    ///    state before calling [`PostThreadMessage`].
    ///  - In the thread to which the message will be posted, call [`PeekMessage`] as shown here to
    ///    force the system to create the message queue:
    ///    `PeekMessage(&msg, null_mut, WM_USER, WM_USER, PM_NOREMOVE)`
    ///  - Set the event, to indicate that the thread is ready to receive posted messages.
    ///
    /// The thread to which the message is posted retrieves the message by calling the
    /// [`GetMessage`] or [`PeekMessage`] function. The `wnd` member of the returned [`MSG`]
    /// structure is [`null_mut`].
    ///
    /// Messages posted by [`PostThreadMessage`] are not associated with a window. As a general
    /// rule, messages that are not associated with a window cannot be dispatched by the
    /// [`DispatchMessage`] function. Therefore, if the recipient thread is in a modal loop (as
    /// used by [`MessageBox`] or [`DialogBox`]), the messages will be lost. To intercept thread
    /// messages while in a modal loop, use a thread-specific hook.
    ///
    /// The system only does marshalling for system messages (those in the range 0 to
    /// `WM_USER - 1`)). To send other messages (those `>= WM_USER`) to another process, you must
    /// do custom marshalling.
    ///
    /// There is a limit of 10,000 posted messages per message queue. This limit should be
    /// sufficiently large. If your application exceeds the limit, it should be redesigned to avoid
    /// consuming so many system resources.
    pub fn PostThreadMessageA(
        thread_id: DWORD,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> BOOL;
}
