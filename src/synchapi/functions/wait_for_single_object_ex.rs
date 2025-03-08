use crate::{BOOL, DWORD, HANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, WaitForMultipleObjectsEx, INFINITE, TRUE, WAIT_ABANDONED, WAIT_FAILED,
    WAIT_IO_COMPLETION, WAIT_OBJECT_0, WAIT_TIMEOUT,
};

#[link(name = "Kernel32")]
extern "system" {
    /// Waits until the specified object is in the signaled state, an I/O completion routine or
    /// asynchronous procedure call (APC) is queued to the thread, or the time-out interval
    /// elapses.
    ///
    /// To wait for multiple objects, use the [`WaitForMultipleObjectsEx`].
    ///
    /// # Parameters
    ///  * `handle` - A handle to the object. For a list of the object types whose handles can be
    ///               specified, see the following Remarks section. If this handle is closed while
    ///               the wait is still pending, the function's behavior is undefined. The handle
    ///               must have the `SYNCHRONIZE` access right.
    ///  * `milliseconds` - The time-out interval, in milliseconds. If a nonzero value is
    ///                     specified, the function waits until the object is signaled, an I/O
    ///                     completion routine or APC is queued, or the interval elapses. If
    ///                     `milliseconds` is zero, the function does not enter a wait state if the
    ///                     criteria is not met; it always returns immediately. If `milliseconds`
    ///                     is [`INFINITE`], the function will return only when the object is
    ///                     signaled or an I/O completion routine or APC is queued.
    ///  * `alertable` - If this parameter is [`TRUE`] and the thread is in the waiting state, the
    ///                  function returns when the system queues an I/O completion routine or APC,
    ///                  and the thread runs the routine or function. Otherwise, the function does
    ///                  not return, and the completion routine or APC function is not executed. A
    ///                  completion routine is queued when the [`ReadFileEx`] or [`WriteFileEx`]
    ///                  function in which it was specified has completed. The wait function
    ///                  returns and the completion routine is called only if `alertable` is
    ///                  [`TRUE`], and the calling thread is the thread that initiated the read or
    ///                  write operation. An APC is queued when you call [`QueueUserAPC`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value indicates the event that caused the function to
    /// return. It can be one of the following values:
    ///  * [`WAIT_ABANDONED`] - The specified object is a mutex object that was not released by the
    ///                         thread that owned the mutex object before the owning thread
    ///                         terminated. Ownership of the mutex object is granted to the calling
    ///                         thread and the mutex is set to nonsignaled. If the mutex was
    ///                         protecting persistent state information, you should check it for
    ///                         consistency.
    ///  * [`WAIT_IO_COMPLETION`] - The wait was ended by one or more user-mode asynchronous
    ///                             procedure calls (APC) queued to the thread.
    ///  * [`WAIT_OBJECT_0`] - The state of the specified object is signaled.
    ///  * [`WAIT_TIMEOUT`] - The time-out interval elapsed, and the object's state is nonsignaled.
    ///  * [`WAIT_FAILED`] - The function has failed. To get extended error information, call
    ///                      [`GetLastError`].
    ///
    /// # Remarks
    /// The [`WaitForSingleObjectEx`] function determines whether the wait criteria have been met.
    /// If the criteria have not been met, the calling thread enters the wait state until the
    /// conditions of the wait criteria have been met or the time-out interval elapses.
    ///
    /// The function modifies the state of some types of synchronization objects. Modification
    /// occurs only for the object whose signaled state caused the function to return. For example,
    /// the count of a semaphore object is decreased by one.
    ///
    /// The [`WaitForSingleObjectEx`] function can wait for the following objects:
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
    /// sent to all windows in the system. A thread that uses a wait function with no time-out
    /// interval may cause the system to become deadlocked. Two examples of code that indirectly
    /// creates windows are DDE and the [`CoInitialize`] function. Therefore, if you have a thread
    /// that creates windows, use [`MsgWaitForMultipleObjects`] or [`MsgWaitForMultipleObjectsEx`],
    /// rather than [`WaitForSingleObjectEx`].
    pub fn WaitForSingleObjectEx(handle: HANDLE, milliseconds: DWORD, alertable: BOOL) -> DWORD;
}
