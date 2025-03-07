use crate::{BOOL, DWORD, HANDLE, LPDWORD, LPOVERLAPPED, PULONG_PTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateIoCompletionPort, GetLastError, ERROR_ABANDONED_WAIT_0, FALSE, INFINITE, OVERLAPPED, TRUE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Attempts to dequeue an I/O completion packet from the specified I/O completion port. If
    /// there is no completion packet queued, the function waits for a pending I/O operation
    /// associated with the completion port to complete.
    ///
    /// To dequeue multiple I/O completion packets at once, use the [`GetQueuedCompletionStatusEx`]
    /// function.
    ///
    /// # Parameters
    ///  * `completion_port` - A handle to the completion port. To create a completion port, use
    ///                        the [`CreateIoCompletionPort`] function.
    ///  * `number_of_bytes_transferred` - A pointer to a variable that receives the number of
    ///                                    bytes transferred in a completed I/O operation.
    ///  * `completion_key` - A pointer to a variable that receives the completion key value
    ///                       associated with the file handle whose I/O operation has completed. A
    ///                       completion key is a per-file key that is specified in a call to
    ///                       [`CreateIoCompletionPort`].
    ///  * `overlapped` - A pointer to a variable that receives the address of the [`OVERLAPPED`]
    ///                   structure that was specified when the completed I/O operation was
    ///                   started. Even if you have passed the function a file handle associated
    ///                   with a completion port and a valid [`OVERLAPPED`] structure, an
    ///                   application can prevent completion port notification. This is done by
    ///                   specifying a valid event handle for the `event` member of the
    ///                   [`OVERLAPPED`] structure, and setting its low-order bit. A valid event
    ///                   handle whose low-order bit is set prevents the completion of the
    ///                   overlapped I/O from enqueing a completion packet to the completion port.
    ///  * `milliseconds` - The number of milliseconds that the caller is willing to wait for a
    ///                     completion packet to appear at the completion port. If a completion
    ///                     packet does not appear within the specified time, the function times
    ///                     out, returns [`FALSE`], and sets `*overlapped` to [`null_mut`]. If
    ///                     `milliseconds` is [`INFINITE`], the function will never time out. If
    ///                     `milliseconds` is zero and there is no I/O operation to dequeue, the
    ///                     function will time out immediately.
    ///
    /// # Return Value
    /// Returns nonzero ([`TRUE`]) if successful or zero ([`FALSE`]) otherwise.
    ///
    /// To get extended error information, call [`GetLastError`].
    ///
    /// # Remarks
    /// This function associates a thread with the specified completion port. A thread can be
    /// associated with at most one completion port.
    ///
    /// If a call to [`GetQueuedCompletionStatus`] fails because the completion port handle
    /// associated with it is closed while the call is outstanding, the function returns [`FALSE`],
    /// `*overlapped` will be [`null_mut`], and [`GetLastError`] will return
    /// [`ERROR_ABANDONED_WAIT_0`].
    ///
    /// Windows Server 2003 and Windows XP: Closing the completion port handle while a call is
    /// outstanding will not result in the previously stated behavior. The function will continue
    /// to wait until an entry is removed from the port or until a time-out occurs, if specified as
    /// a value other than [`INFINITE`].
    ///
    /// If the [`GetQueuedCompletionStatus`] function succeeds, it dequeued a completion packet for
    /// a successful I/O operation from the completion port and has stored information in the
    /// variables pointed to by the following parameters: `number_of_bytes`, `completion_key`, and
    /// `overlapped`. Upon failure (the return value is [`FALSE`]), those same parameters can
    /// contain particular value combinations as follows:
    ///  - If `*overlapped` is [`null_mut`], the function did not dequeue a completion packet from
    ///    the completion port. In this case, the function does not store information in the
    ///    variables pointed to by the `number_of_bytes` and `completion_key` parameters, and their
    ///    values are indeterminate.
    ///  - If `*overlapped` is not [`null_mut`] and the function dequeues a completion packet for a
    ///    failed I/O operation from the completion port, the function stores information about the
    ///    failed operation in the variables pointed to by `number_of_bytes`, `completion_key`, and
    ///    `overlapped`. To get extended error information, call [`GetLastError`].
    pub fn GetQueuedCompletionStatus(
        completion_port: HANDLE,
        number_of_bytes_transferred: LPDWORD,
        completion_key: PULONG_PTR,
        overlapped: *mut LPOVERLAPPED,
        milliseconds: DWORD,
    ) -> BOOL;
}
