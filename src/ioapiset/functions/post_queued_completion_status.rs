use crate::{BOOL, DWORD, HANDLE, LPOVERLAPPED, ULONG_PTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, GetQueuedCompletionStatus, OVERLAPPED};

#[link(name = "Kernel32")]
extern "system" {
    /// Posts an I/O completion packet to an I/O completion port.
    ///
    /// # Parameters
    ///  * `completion_port` - A handle to an I/O completion port to which the I/O completion
    ///                        packet is to be posted.
    ///  * `number_of_bytes_transferred` - The value to be returned through the
    ///                                    `number_of_bytes_transferred` parameter of the
    ///                                    [`GetQueuedCompletionStatus`] function.
    ///  * `completion_key` - The value to be returned through the `completion_key` parameter of
    ///                       the [`GetQueuedCompletionStatus`] function.
    ///  * `overlapped` - The value to be returned through the `overlapped` parameter of the
    ///                   [`GetQueuedCompletionStatus`] function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// The I/O completion packet will satisfy an outstanding call to the
    /// [`GetQueuedCompletionStatus`] function. This function returns with the three values passed
    /// as the second, third, and fourth parameters of the call to [`PostQueuedCompletionStatus`].
    /// The system does not use or validate these values. In particular, the `overlapped` parameter
    /// need not point to an [`OVERLAPPED`] structure.
    pub fn PostQueuedCompletionStatus(
        completion_port: HANDLE,
        number_of_bytes_transferred: DWORD,
        completion_key: ULONG_PTR,
        overlapped: LPOVERLAPPED,
    ) -> BOOL;
}
