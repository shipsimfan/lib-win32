use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, GetMessage, PeekMessage, WaitForMultipleObjectsEx};

#[link(name = "User32")]
extern "system" {
    /// Blocks thread execution until the thread needs to process a new message. The new message
    /// could be an input message, a queued message, or a non-queued message.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// Note that [`WaitMessage`] does not return for unprocessed messages reported by a previous
    /// function which checks the queue. This is because functions such as [`PeekMessage`],
    /// [`GetMessage`], [`GetQueueStatus`], [`WaitMessage`], [`MsgWaitForMultipleObjects`], and
    /// [`MsgWaitForMultipleObjectsEx`] check the queue and then change the state information for
    /// the queue so that the message is no longer considered new. A subsequent call to
    /// [`WaitMessage`] will not return until new messages arrive. The existing unprocessed
    /// messages (received prior to the last time the thread checked the queue) are not considered
    /// to be new.
    pub fn WaitMessage() -> BOOL;
}
