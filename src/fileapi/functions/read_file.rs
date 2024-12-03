use crate::{BOOL, DWORD, HANDLE, LPDWORD, LPOVERLAPPED, LPVOID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateFile, GetCommTimeouts, GetLastError, SetCommTimeouts, ERROR_BROKEN_PIPE,
    ERROR_HANDLE_EOF, ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_USER_BUFFER, ERROR_IO_PENDING,
    ERROR_MORE_DATA, ERROR_NOT_ENOUGH_MEMORY, ERROR_NOT_ENOUGH_QUOTA, ERROR_OPERATION_ABORTED,
    FALSE, FILE_FLAG_NO_BUFFERING, FILE_FLAG_OVERLAPPED, OVERLAPPED, TRUE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Reads data from the specified file or input/output (I/O) device. Reads occur at the
    /// position specified by the file pointer if supported by the device.
    ///
    /// This function is designed for both synchronous and asynchronous operations. For a similar
    /// function designed solely for asynchronous operation, see [`ReadFileEx`].
    ///
    /// # Parameters
    ///  * `file` - A handle to the device (for example, a file, file stream, physical disk,
    ///             volume, console buffer, tape drive, socket, communications resource, mailslot,
    ///             or pipe). The `file` parameter must have been created with read access. For
    ///             asynchronous read operations, `file` can be any handle that is opened with the
    ///             [`FILE_FLAG_OVERLAPPED`] flag by the [`CreateFile`] function, or a socket
    ///             handle returned by the socket or accept function.
    ///  * `buffer` - A pointer to the buffer that receives the data read from a file or device.
    ///               This buffer must remain valid for the duration of the read operation. The
    ///               caller must not use this buffer until the read operation is completed.
    ///  * `number_of_bytes_to_read` - The maximum number of bytes to be read.
    ///  * `number_of_bytes_read` - A pointer to the variable that receives the number of bytes
    ///                             read when using a synchronous `file` parameter. [`ReadFile`]
    ///                             sets this value to zero before doing any work or error
    ///                             checking. Use [`null_mut`] for this parameter if this is an
    ///                             asynchronous operation to avoid potentially erroneous results.
    ///                             This parameter can be [`null_mut`] only when the `overlapped`
    ///                             parameter is not [`null_mut`]. Windows 7: This parameter can
    ///                             not be [`null_mut`].
    ///  * `overlapped` - A pointer to an [`OVERLAPPED`] structure is required if the `file`
    ///                   parameter was opened with [`FILE_FLAG_OVERLAPPED`], otherwise it can be
    ///                   [`null_mut`]. If `file` is opened with [`FILE_FLAG_OVERLAPPED`], the
    ///                   `overlapped` parameter must point to a valid and unique [`OVERLAPPED`]
    ///                   structure, otherwise the function can incorrectly report that the read
    ///                   operation is complete. For an `file` that supports byte offsets, if you
    ///                   use this parameter you must specify a byte offset at which to start
    ///                   reading from the file or device. This offset is specified by setting the
    ///                   `offset` and `offset_high` members of the [`OVERLAPPED`] structure. For
    ///                   an `file` that does not support byte offsets, `offset` and `offset_high`
    ///                   are ignored.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero ([`TRUE`]).
    ///
    /// If the function fails, or is completing asynchronously, the return value is zero
    /// ([`FALSE`]). To get extended error information, call the [`GetLastError`] function.
    ///
    /// # Remarks
    /// The [`ReadFile`] function returns when one of the following conditions occur:
    ///  - The number of bytes requested is read.
    ///  - A write operation completes on the write end of the pipe.
    ///  - An asynchronous handle is being used and the read is occurring asynchronously.
    ///  - An error occurs.
    ///
    /// The [`ReadFile`] function may fail with [`ERROR_INVALID_USER_BUFFER`] or
    /// [`ERROR_NOT_ENOUGH_MEMORY`] whenever there are too many outstanding asynchronous I/O
    /// requests.
    ///
    /// To cancel all pending asynchronous I/O operations, use either:
    ///  - [`CancelIo`]: This function only cancels operations issued by the calling thread for the
    ///                  specified file handle.
    ///  - [`CancelIoEx`]: This function cancels all operations issued by the threads for the
    ///                    specified file handle.
    ///
    /// Use [`CancelSynchronousIo`] to cancel pending synchronous I/O operations.
    ///
    /// I/O operations that are canceled complete with the error [`ERROR_OPERATION_ABORTED`].
    ///
    /// The [`ReadFile`] function may fail with [`ERROR_NOT_ENOUGH_QUOTA`], which means the calling
    /// process's buffer could not be page-locked. For additional information, see
    /// [`SetProcessWorkingSetSize`].
    ///
    /// If part of a file is locked by another process and the read operation overlaps the locked
    /// portion, this function fails.
    ///
    /// Accessing the input buffer while a read operation is using the buffer may lead to
    /// corruption of the data read into that buffer. Applications must not read from, write to,
    /// reallocate, or free the input buffer that a read operation is using until the read
    /// operation completes. This can be particularly problematic when using an asynchronous file
    /// handle.
    ///
    /// Characters can be read from the console input buffer by using [`ReadFile`] with a handle to
    /// console input. The console mode determines the exact behavior of the [`ReadFile`] function.
    /// By default, the console mode is [`ENABLE_LINE_INPUT`], which indicates that [`ReadFile`]
    /// should read until it reaches a carriage return. If you press `Ctrl+C`, the call succeeds,
    /// but [`GetLastError`] returns [`ERROR_OPERATION_ABORTED`]. For more information, see
    /// [`CreateFile`].
    ///
    /// When reading from a communications device, the behavior of [`ReadFile`] is determined by
    /// the current communication time-out as set and retrieved by using the [`SetCommTimeouts`]
    /// and [`GetCommTimeouts`] functions. Unpredictable results can occur if you fail to set the
    /// time-out values.
    ///
    /// If [`ReadFile`] attempts to read from a mailslot that has a buffer that is too small, the
    /// function returns [`FALSE`] and [`GetLastError`] returns [`ERROR_INSUFFICIENT_BUFFER`].
    ///
    /// There are strict requirements for successfully working with files opened with
    /// [`CreateFile`] using the [`FILE_FLAG_NO_BUFFERING`] flag.
    ///
    /// If `file` was opened with [`FILE_FLAG_OVERLAPPED`], the following conditions are in effect:
    ///  - The `overlapped` parameter must point to a valid and unique [`OVERLAPPED`] structure,
    ///    otherwise the function can incorrectly report that the read operation is complete.
    ///  - The `number_of_bytes_read` parameter should be set to [`null_mut`]. Use the
    ///    [`GetOverlappedResult`] function to get the actual number of bytes read. If the `file`
    ///    parameter is associated with an I/O completion port, you can also get the number of
    ///    bytes read by calling the [`GetQueuedCompletionStatus`] function.
    ///
    /// ## Synchronization and File Position
    /// If `file` is opened with [`FILE_FLAG_OVERLAPPED`], it is an asynchronous file handle;
    /// otherwise it is synchronous. The rules for using the [`OVERLAPPED`] structure are slightly
    /// different for each, as previously noted.
    ///
    /// Considerations for working with asynchronous file handles:
    ///  - [`ReadFile`] may return before the read operation is complete. In this scenario,
    ///    [`ReadFile`] returns [`FALSE`] and the [`GetLastError`] function returns
    ///    [`ERROR_IO_PENDING`], which allows the calling process to continue while the system
    ///    completes the read operation.
    ///  - The `overlapped` parameter must not be [`null_mut`] and should be used with the
    ///    following facts in mind:
    ///    - Although the event specified in the [`OVERLAPPED`] structure is set and reset
    ///      automatically by the system, the offset that is specified in the [`OVERLAPPED`]
    ///      structure is not automatically updated.
    ///    - [`ReadFile`] resets the event to a nonsignaled state when it begins the I/O operation.
    ///    - The event specified in the [`OVERLAPPED`] structure is set to a signaled state when
    ///      the read operation is complete; until that time, the read operation is considered
    ///      pending.
    ///    - Because the read operation starts at the offset that is specified in the
    ///      [`OVERLAPPED`] structure, and ReadFile may return before the system-level read
    ///      operation is complete (read pending), neither the offset nor any other part of the
    ///      structure should be modified, freed, or reused by the application until the event is
    ///      signaled (that is, the read completes).
    ///    - If end-of-file (EOF) is detected during asynchronous operations, the call to
    ///      [`GetOverlappedResult`] for that operation returns [`FALSE`] and [`GetLastError`]
    ///      returns [`ERROR_HANDLE_EOF`].
    ///
    /// Considerations for working with synchronous file handles:
    ///  - If `overlapped` is [`null_mut`], the read operation starts at the current file position
    ///    and [`ReadFile`] does not return until the operation is complete, and the system updates
    ///    the file pointer before [`ReadFile`] returns.
    ///  - If `overlapped` is not [`null_mut`], the read operation starts at the offset that is
    ///    specified in the [`OVERLAPPED`] structure and [`ReadFile`] does not return until the
    ///    read operation is complete. The system updates the [`OVERLAPPED`] offset and the file
    ///    pointer before [`ReadFile`] returns.
    ///  - If `overlapped` is [`null_mut`], then when a synchronous read operation reaches the end
    ///    of a file, [`ReadFile`] returns [`TRUE`] and sets `*number_of_bytes_read` to zero.
    ///  - If `overlapped` is not [`null_mut`], then when a synchronous read operation reaches the
    ///    end of a file, [`ReadFile`] returns [`FALSE`] and [`GetLastError`] returns
    ///    [`ERROR_HANDLE_EOF`].
    ///
    /// ## Pipes
    /// If an anonymous pipe is being used and the write handle has been closed, when [`ReadFile`]
    /// attempts to read using the pipe's corresponding read handle, the function returns [`FALSE`]
    /// and [`GetLastError`] returns [`ERROR_BROKEN_PIPE`].
    ///
    /// If a named pipe is being read in message mode and the next message is longer than the
    /// `number_of_bytes_read` parameter specifies, [`ReadFile`] returns [`FALSE`] and
    /// [`GetLastError`] returns [`ERROR_MORE_DATA`]. The remainder of the message can be read by a
    /// subsequent call to the [`ReadFile`] or [`PeekNamedPipe`] function.
    ///
    /// If the `number_of_bytes_read` parameter is zero when [`ReadFile`] returns [`TRUE`] on a
    /// pipe, the other end of the pipe called the [`WriteFile`] function with
    /// `number_of_bytes_to_write` set to zero.
    ///
    /// ## Transacted Operations
    /// If there is a transaction bound to the file handle, then the function returns data from the
    /// transacted view of the file. A transacted read handle is guaranteed to show the same view
    /// of a file for the duration of the handle.
    pub fn ReadFile(
        file: HANDLE,
        buffer: LPVOID,
        number_of_bytes_to_read: DWORD,
        number_of_bytes_read: LPDWORD,
        overlapped: LPOVERLAPPED,
    ) -> BOOL;
}
