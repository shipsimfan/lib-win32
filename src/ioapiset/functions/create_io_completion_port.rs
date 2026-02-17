use crate::{DWORD, HANDLE, ULONG_PTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CloseHandle, CreateFile, GetLastError, GetQueuedCompletionStatus, PostQueuedCompletionStatus,
    FILE_FLAG_OVERLAPPED, INVALID_HANDLE_VALUE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Creates an input/output (I/O) completion port and associates it with a specified file
    /// handle, or creates an I/O completion port that is not yet associated with a file handle,
    /// allowing association at a later time.
    ///
    /// Associating an instance of an opened file handle with an I/O completion port allows a
    /// process to receive notification of the completion of asynchronous I/O operations involving
    /// that file handle.
    ///
    /// # Parameters
    ///  * `file_handle` - An open file handle or [`INVALID_HANDLE_VALUE`]. he handle must be to an
    ///                    object that supports overlapped I/O. If a handle is provided, it has to
    ///                    have been opened for overlapped I/O completion. For example, you must
    ///                    specify the [`FILE_FLAG_OVERLAPPED`] flag when using the [`CreateFile`]
    ///                    function to obtain the handle. If [`INVALID_HANDLE_VALUE`] is specified,
    ///                    the function creates an I/O completion port without associating it with
    ///                    a file handle. In this case, the `existing_completion_port` parameter
    ///                    must be [`null_mut`] and the `completion_key` parameter is ignored.
    ///  * `existing_completion_port` - A handle to an existing I/O completion port or
    ///                                 [`null_mut`]. If this parameter specifies an existing I/O
    ///                                 completion port, the function associates it with the handle
    ///                                 specified by the `file_handle` parameter. The function
    ///                                 returns the handle of the existing I/O completion port if
    ///                                 successful; it does not create a new I/O completion port.
    ///                                 If this parameter is [`null_mut`], the function creates a
    ///                                 new I/O completion port and, if the `file_handle` parameter
    ///                                 is valid, associates it with the new I/O completion port.
    ///                                 Otherwise no file handle association occurs. The function
    ///                                 returns the handle to the new I/O completion port if
    ///                                 successful.
    ///  * `completion_key` - The per-handle user-defined completion key that is included in every
    ///                       I/O completion packet for the specified file handle. For more
    ///                       information, see the Remarks section.
    ///  * `number_of_concurrent_threads` - The maximum number of threads that the operating system
    ///                                     can allow to concurrently process I/O completion
    ///                                     packets for the I/O completion port. This parameter is
    ///                                     ignored if the `existing_completion_port` parameter is
    ///                                     not [`null_mut`]. If this parameter is zero, the system
    ///                                     allows as many concurrently running threads as there
    ///                                     are processors in the system.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the handle to an I/O completion port:
    ///  - If the `existing_completion_port` parameter was [`null_mut`], the return value is a new
    ///    handle.
    ///  - If the `existing_completion_port` parameter was a valid I/O completion port handle, the
    ///    return value is that same handle.
    ///  - If the `file_handle` parameter was a valid handle, that file handle is now associated
    ///    with the returned I/O completion port.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call the [`GetLastError`] function.
    ///
    /// # Remarks
    /// The I/O system can be instructed to send I/O completion notification packets to I/O
    /// completion ports, where they are queued. The [`CreateIoCompletionPort`] function provides
    /// this functionality.
    ///
    /// An I/O completion port and its handle are associated with the process that created it and
    /// is not sharable between processes. However, a single handle is sharable between threads in
    /// the same process.
    ///
    /// [`CreateIoCompletionPort`] can be used in three distinct modes:
    ///  - Create only an I/O completion port without associating it with a file handle.
    ///  - Associate an existing I/O completion port with a file handle.
    ///  - Perform both creation and association in a single call.
    ///
    /// To create an I/O completion port without associating it, set the `file_handle` parameter to
    /// [`INVALID_HANDLE_VALUE`], the `existing_completion_port` parameter to [`null_mut`], and the
    /// `completion_key` parameter to zero (which is ignored in this case). Set the
    /// `number_of_concurrent_threads` parameter to the desired concurrency value for the new I/O
    /// completion port, or zero for the default (the number of processors in the system).
    ///
    /// The handle passed in the `file_handle` parameter can be any handle that supports overlapped
    /// I/O. Most commonly, this is a handle opened by the [`CreateFile`] function using the
    /// [`FILE_FLAG_OVERLAPPED`] flag (for example, files, mail slots, and pipes). Objects created
    /// by other functions such as socket can also be associated with an I/O completion port. For
    /// an example using sockets, see [`AcceptEx`]. A handle can be associated with only one I/O
    /// completion port, and after the association is made, the handle remains associated with that
    /// I/O completion port until it is closed.
    ///
    /// Multiple file handles can be associated with a single I/O completion port by calling
    /// [`CreateIoCompletionPort`] multiple times with the same I/O completion port handle in the
    /// `existing_completion_port` parameter and a different file handle in the `file_handle`
    /// parameter each time.
    ///
    /// Use the `completion_key` parameter to help your application track which I/O operations have
    /// completed. This value is not used by [`CreateIoCompletionPort`] for functional control;
    /// rather, it is attached to the file handle specified in the `file_handle` parameter at the
    /// time of association with an I/O completion port. This completion key should be unique for
    /// each file handle, and it accompanies the file handle throughout the internal completion
    /// queuing process. It is returned in the [`GetQueuedCompletionStatus`] function call when a
    /// completion packet arrives. The `completion_key` parameter is also used by the
    /// [`PostQueuedCompletionStatus`] function to queue your own special-purpose completion
    /// packets.
    ///
    /// After an instance of an open handle is associated with an I/O completion port, it cannot be
    /// used in the [`ReadFileEx`] or [`WriteFileEx`] function because these functions have their
    /// own asynchronous I/O mechanisms.
    ///
    /// It is best not to share a file handle associated with an I/O completion port by using
    /// either handle inheritance or a call to the [`DuplicateHandle`] function. Operations
    /// performed with such duplicate handles generate completion notifications. Careful
    /// consideration is advised.
    ///
    /// The I/O completion port handle and every file handle associated with that particular I/O
    /// completion port are known as references to the I/O completion port. The I/O completion port
    /// is released when there are no more references to it. Therefore, all of these handles must
    /// be properly closed to release the I/O completion port and its associated system resources.
    /// After these conditions are satisfied, close the I/O completion port handle by calling the
    /// [`CloseHandle`] function.
    pub fn CreateIoCompletionPort(
        file_handle: HANDLE,
        existing_completion_port: HANDLE,
        completion_key: ULONG_PTR,
        number_of_concurrent_threads: DWORD,
    ) -> HANDLE;
}
