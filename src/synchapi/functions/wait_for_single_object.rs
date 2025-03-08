use crate::{DWORD, HANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, INFINITE, WAIT_ABANDONED, WAIT_FAILED, WAIT_OBJECT_0, WAIT_TIMEOUT};

#[link(name = "Kernel32")]
extern "system" {
    /// Waits until the specified object is in the signaled state or the time-out interval elapses.
    ///
    /// To enter an alertable wait state, use the [`WaitForSingleObjectEx`] function. To wait for
    /// multiple objects, use [`WaitForMultipleObjects`].
    ///
    /// # Parameters
    ///  * `handle` - A handle to the object. For a list of the object types whose handles can be
    ///               specified, see the following Remarks section. If this handle is closed while
    ///               the wait is still pending, the function's behavior is undefined. The handle
    ///               must have the `SYNCHRONIZE` access right.
    ///  * `milliseconds` - The time-out interval, in milliseconds. If a nonzero value is
    ///               specified, the function waits until the object is signaled or the interval
    ///               elapses. If `milliseconds` is zero, the function does not enter a wait state
    ///               if the object is not signaled; it always returns immediately. If
    ///               `milliseconds` is [`INFINITE`], the function will return only when the object
    ///               is signaled.
    ///
    /// # Return Value
    /// If the function succeeds, the return value indicates the event that caused the function to
    /// return. It can be one of the following values:
    ///  * [`WAIT_ABANDONED`] - The specified object is a mutex object that was not released by the
    ///                         thread that owned the mutex object before the owning thread
    ///                         terminated. Ownership of the mutex object is granted to the calling
    ///                         thread and the mutex state is set to nonsignaled. If the mutex was
    ///                         protecting persistent state information, you should check it for
    ///                         consistency.
    ///  * [`WAIT_OBJECT_0`] - The state of the specified object is signaled.
    ///  * [`WAIT_TIMEOUT`] - The time-out interval elapsed, and the object's state is nonsignaled.
    ///  * [`WAIT_FAILED`] - The function has failed. To get extended error information, call
    ///                      [`GetLastError`].
    ///
    /// # Remarks
    /// The [`WaitForSingleObject`] function checks the current state of the specified object. If
    /// the object's state is nonsignaled, the calling thread enters the wait state until the
    /// object is signaled or the time-out interval elapses.
    ///
    /// The function modifies the state of some types of synchronization objects. Modification
    /// occurs only for the object whose signaled state caused the function to return. For example,
    /// the count of a semaphore object is decreased by one.
    ///
    /// The WaitForSingleObject function can wait for the following objects:
    ///  - Change notification
    ///  - Console input
    ///  - Event
    ///  - Memory resource notification
    ///  - Mutex
    ///  - Process
    ///  - Semaphore
    ///  - Thread
    ///  - Waitable timer
    ///
    /// Use caution when calling the wait functions and code that directly or indirectly creates
    /// windows. If a thread creates any windows, it must process messages. Message broadcasts are
    /// sent to all windows in the system. A thread that uses a wait function with an [`INFINITE`]
    /// time-out interval may cause the system to become deadlocked. Two examples of code that
    /// indirectly creates windows are DDE and the [`CoInitialize`] function. Therefore, if you
    /// have a thread that creates windows, use [`MsgWaitForMultipleObjects`] or
    /// [`MsgWaitForMultipleObjectsEx`], rather than [`WaitForSingleObject`].
    pub fn WaitForSingleObject(handle: HANDLE, milliseconds: DWORD) -> DWORD;
}
