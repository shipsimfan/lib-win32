use crate::{BOOL, HANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, ERROR_OPERATION_ABORTED, FILE_FLAG_OVERLAPPED};

#[link(name = "Kernel32")]
extern "system" {
    /// Cancels all pending input and output (I/O) operations that are issued by the calling thread
    /// for the specified file. The function does not cancel I/O operations that other threads
    /// issue for a file handle.
    ///
    /// To cancel I/O operations from another thread, use the [`CancelIoEx`] function.
    ///
    /// # Parameters
    ///  * `file` - A handle to the file. The function cancels all pending I/O operations for this
    ///             file handle.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero. The cancel operation for all pending
    /// I/O operations issued by the calling thread for the specified file handle was successfully
    /// requested. The thread can use the GetOverlappedResult function to determine when the I/O
    /// operations themselves have been completed.
    ///
    /// If the function fails, the return value is zero (0). To get extended error information,
    /// call the [`GetLastError`] function.
    ///
    /// # Remarks
    /// If there are any pending I/O operations in progress for the specified file handle, and they
    /// are issued by the calling thread, the [`CancelIo`] function cancels them. [`CancelIo`]
    /// cancels only outstanding I/O on the handle, it does not change the state of the handle;
    /// this means that you cannot rely on the state of the handle because you cannot know whether
    /// the operation was completed successfully or canceled.
    ///
    /// The I/O operations must be issued as overlapped I/O. If they are not, the I/O operations do
    /// not return to allow the thread to call the [`CancelIo`] function. Calling the [`CancelIo`]
    /// function with a file handle that is not opened with [`FILE_FLAG_OVERLAPPED`] does nothing.
    ///
    /// All I/O operations that are canceled complete with the error [`ERROR_OPERATION_ABORTED`],
    /// and all completion notifications for the I/O operations occur normally.
    pub fn CancelIo(file: HANDLE) -> BOOL;
}
