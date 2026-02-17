use crate::{BOOL, DWORD, HANDLE, LPCVOID, LPDWORD, LPOVERLAPPED};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateFile, GetCommTimeouts, GetLastError, SetCommTimeouts, COMMTIMEOUTS,
    ERROR_INVALID_USER_BUFFER, ERROR_NOT_ENOUGH_MEMORY, ERROR_NOT_ENOUGH_QUOTA,
    ERROR_OPERATION_ABORTED, FALSE, FILE_FLAG_NO_BUFFERING, FILE_FLAG_OVERLAPPED,
    FILE_FLAG_WRITE_THROUGH, OVERLAPPED, TRUE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Writes data to the specified file or input/output (I/O) device.
    ///
    /// This function is designed for both synchronous and asynchronous operation. For a similar
    /// function designed solely for asynchronous operation, see [`WriteFileEx`].
    ///
    /// # Parameters
    ///  * `file` - A handle to the file or I/O device (for example, a file, file stream, physical
    ///             disk, volume, console buffer, tape drive, socket, communications resource,
    ///             mailslot, or pipe). The `file` parameter must have been created with the write
    ///             access. For asynchronous write operations, `file` can be any handle opened with
    ///             the [`CreateFile`] function using the [`FILE_FLAG_OVERLAPPED`] flag or a socket
    ///             handle returned by the socket or accept function.
    ///  * `buffer` - A pointer to the buffer containing the data to be written to the file or
    ///               device. This buffer must remain valid for the duration of the write
    ///               operation. The caller must not use this buffer until the write operation is
    ///               completed.
    ///  * `number_of_bytes_to_write` - The number of bytes to be written to the file or device. A
    ///                                 value of zero specifies a null write operation. The
    ///                                 behavior of a null write operation depends on the
    ///                                 underlying file system or communications technology.
    ///                                 Windows Server 2003 and Windows XP:  Pipe write operations
    ///                                 across a network are limited in size per write. The amount
    ///                                 varies per platform. For x86 platforms it's 63.97 MB. For
    ///                                 x64 platforms it's 31.97 MB. For Itanium it's 63.95 MB. For
    ///                                 more information regarding pipes, see the Remarks section.
    ///  * `number_of_bytes_written` - A pointer to the variable that receives the number of bytes
    ///                                written when using a synchronous `file` parameter.
    ///                                [`WriteFile`] sets this value to zero before doing any work
    ///                                or error checking. Use [`null_mut`] for this parameter if
    ///                                this is an asynchronous operation to avoid potentially
    ///                                erroneous results. This parameter can be [`null_mut`] only
    ///                                when the `overlapped` parameter is not [`null_mut`]. Windows
    ///                                7:  This parameter can not be [`null_mut`]. For more
    ///                                information, see the Remarks section.
    ///  * `overlapped` - A pointer to an [`OVERLAPPED`] structure is required if the `file` '
    ///                   parameter was opened with [`FILE_FLAG_OVERLAPPED`], otherwise this
    ///                   parameter can be [`null_mut`]. For a `file` that supports byte offsets,
    ///                   if you use this parameter you must specify a byte offset at which to
    ///                   start writing to the file or device. This offset is specified by setting
    ///                   the `offset` and `offset_high` members of the [`OVERLAPPED`] structure.0
    ///                   For a `file` that does not support byte offsets, `offset` and
    ///                   `offset_high` are ignored. To write to the end of file, specify both the
    ///                   `offset` and `offset_high` members of the [`OVERLAPPED`] structure as
    ///                   `0xFFFFFFFF`. This is functionally equivalent to previously calling the
    ///                   [`CreateFile`] function to open `file` using [`FILE_APPEND_DATA`] access.
    ///                   For more information about different combinations of `overlapped` and
    ///                   [`FILE_FLAG_OVERLAPPED`], see the Remarks section.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero ([`TRUE`]).
    ///
    /// If the function fails, or is completing asynchronously, the return value is zero
    /// ([`FALSE`]). To get extended error information, call the [`GetLastError`] function.
    ///
    /// # Remarks
    /// The [`WriteFile`] function returns when one of the following conditions occur:
    ///  - The number of bytes requested is written.
    ///  - A read operation releases buffer space on the read end of the pipe (if the write was
    ///    blocked). For more information, see the Pipes section.
    ///  - An asynchronous handle is being used and the write is occurring asynchronously.
    ///  - An error occurs.
    ///
    /// The [`WriteFile`] function may fail with [`ERROR_INVALID_USER_BUFFER`] or
    /// [`ERROR_NOT_ENOUGH_MEMORY`] whenever there are too many outstanding asynchronous I/O
    /// requests.
    ///
    /// To cancel all pending asynchronous I/O operations, use either:
    ///  - [`CancelIo`] — This function cancels only operations issued by the calling thread for
    ///                   the specified file handle.
    ///  - [`CancelIoEx`] — This function cancels all operations issued by the threads for the
    ///                     specified file handle.
    ///
    /// Use the [`CancelSynchronousIo`] function to cancel pending synchronous I/O operations.
    ///
    /// I/O operations that are canceled complete with the error [`ERROR_OPERATION_ABORTED`].
    ///
    /// The [`WriteFile`] function may fail with [`ERROR_NOT_ENOUGH_QUOTA`], which means the
    /// calling process's buffer could not be page-locked. For more information, see
    /// [`SetProcessWorkingSetSize`].
    ///
    /// If part of the file is locked by another process and the write operation overlaps the
    /// locked portion, [`WriteFile`] fails.
    ///
    /// When writing to a file, the last write time is not fully updated until all handles used for
    /// writing have been closed. Therefore, to ensure an accurate last write time, close the file
    /// handle immediately after writing to the file.
    ///
    /// Accessing the output buffer while a write operation is using the buffer may lead to
    /// corruption of the data written from that buffer. Applications must not write to,
    /// reallocate, or free the output buffer that a write operation is using until the write
    /// operation completes. This can be particularly problematic when using an asynchronous file
    /// handle.
    ///
    /// Note that the time stamps may not be updated correctly for a remote file. To ensure
    /// consistent results, use unbuffered I/O.
    ///
    /// The system interprets zero bytes to write as specifying a null write operation and
    /// [`WriteFile`] does not truncate or extend the file. To truncate or extend a file, use the
    /// [`SetEndOfFile`] function.
    ///
    /// Characters can be written to the screen buffer using [`WriteFile`] with a handle to console
    /// output. The exact behavior of the function is determined by the console mode. The data is
    /// written to the current cursor position. The cursor position is updated after the write
    /// operation. For more information about console handles, see [`CreateFile`].
    ///
    /// When writing to a communications device, the behavior of [`WriteFile`] is determined by the
    /// current communication time-out as set and retrieved by using the [`SetCommTimeouts`] and
    /// [`GetCommTimeouts`] functions. Unpredictable results can occur if you fail to set the
    /// time-out values. For more information about communication time-outs, see [`COMMTIMEOUTS`].
    ///
    /// Although a single-sector write is atomic, a multi-sector write is not guaranteed to be
    /// atomic unless you are using a transaction (that is, the handle created is a transacted
    /// handle; for example, a handle created using [`CreateFileTransacted`]). Multi-sector writes
    /// that are cached may not always be written to the disk right away; therefore, specify
    /// [`FILE_FLAG_WRITE_THROUGH`] in [`CreateFile`] to ensure that an entire multi-sector write
    /// is written to the disk without potential caching delays.
    ///
    /// If you write directly to a volume that has a mounted file system, you must first obtain
    /// exclusive access to the volume. Otherwise, you risk causing data corruption or system
    /// instability, because your application's writes may conflict with other changes coming from
    /// the file system and leave the contents of the volume in an inconsistent state. To prevent
    /// these problems, the following changes have been made in Windows Vista and later:
    ///  - A write on a volume handle will succeed if the volume does not have a mounted file
    ///    system, or if one of the following conditions is true:
    ///    - The sectors to be written to are boot sectors.
    ///    - The sectors to be written to reside outside of file system space.
    ///    - You have explicitly locked or dismounted the volume by using [`FSCTL_LOCK_VOLUME`] or
    ///      [`FSCTL_DISMOUNT_VOLUME`].
    ///    - The volume has no actual file system. (In other words, it has a RAW file system
    ///      mounted.)
    ///  - A write on a disk handle will succeed if one of the following conditions is true:
    ///    - The sectors to be written to do not fall within a volume's extents.
    ///    - The sectors to be written to fall within a mounted volume, but you have explicitly
    ///      locked or dismounted the volume by using [`FSCTL_LOCK_VOLUME`] or
    ///      [`FSCTL_DISMOUNT_VOLUME`].
    ///    - The sectors to be written to fall within a volume that has no mounted file system
    ///      other than RAW.
    ///
    /// There are strict requirements for successfully working with files opened with
    /// [`CreateFile`] using [`FILE_FLAG_NO_BUFFERING`].
    ///
    /// If file was opened with [`FILE_FLAG_OVERLAPPED`], the following conditions are in effect:
    ///  - The `overlapped` parameter must point to a valid and unique [`OVERLAPPED`] structure,
    ///    otherwise the function can incorrectly report that the write operation is complete.
    ///  - The `number_of_bytes_written` parameter should be set to [`null_mut`]. To get the number
    ///    of bytes written, use the [`GetOverlappedResult`] function. If the `file` parameter is
    ///    associated with an I/O completion port, you can also get the number of bytes written by
    ///    calling the [`GetQueuedCompletionStatus`] function.
    ///
    /// In Windows Server 2012, this function is supported by the following technologies.
    pub fn WriteFile(
        file: HANDLE,
        buffer: LPCVOID,
        number_of_bytes_to_write: DWORD,
        number_of_bytes_written: LPDWORD,
        overlapped: LPOVERLAPPED,
    ) -> BOOL;
}
