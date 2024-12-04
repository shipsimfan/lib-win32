use crate::{BOOL, HANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CreateFile, GetLastError, WriteFile, ERROR_INVALID_HANDLE, FALSE, FILE_FLAG_NO_BUFFERING,
    FILE_FLAG_WRITE_THROUGH, GENERIC_WRITE,
};

#[link(name = "Kernel32")]
extern "system" {
    /// Flushes the buffers of a specified file and causes all buffered data to be written to a
    /// file.
    ///
    /// # Parameters
    ///  * `file` - A handle to the open file. The file handle must have the [`GENERIC_WRITE`]
    ///             access right. If `file` is a handle to a communications device, the function
    ///             only flushes the transmit buffer. If `file` is a handle to the server end of a
    ///             named pipe, the function does not return until the client has read all buffered
    ///             data from the pipe.
    ///  
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// The function fails if `file` is a handle to the console output. That is because the console
    /// output is not buffered. The function returns [`FALSE`], and [`GetLastError`] returns
    /// [`ERROR_INVALID_HANDLE`].
    ///
    /// # Remarks
    /// Typically the [`WriteFile`] and [`WriteFileEx`] functions write data to an internal buffer
    /// that the operating system writes to a disk or communication pipe on a regular basis. The
    /// [`FlushFileBuffers`] function writes all the buffered information for a specified file to
    /// the device or pipe.
    ///
    /// Due to disk caching interactions within the system, the [`FlushFileBuffers`] function can
    /// be inefficient when used after every write to a disk drive device when many writes are
    /// being performed separately. If an application is performing multiple writes to disk and
    /// also needs to ensure critical data is written to persistent media, the application should
    /// use unbuffered I/O instead of frequently calling [`FlushFileBuffers`]. To open a file for
    /// unbuffered I/O, call the [`CreateFile`] function with the [`FILE_FLAG_NO_BUFFERING`] and
    /// [`FILE_FLAG_WRITE_THROUGH`] flags. This prevents the file contents from being cached and
    /// flushes the metadata to disk with each write. For more information, see [`CreateFile`].
    ///
    /// To flush all open files on a volume, call [`FlushFileBuffers`] with a handle to the volume.
    /// The caller must have administrative privileges.
    ///
    /// When opening a volume with [`CreateFile`], the `file_name` string should be the following
    /// form:" \\.\x:" or "\\?\Volume{GUID}". Do not use a trailing backslash in the volume name,
    /// because that indicates the root directory of a drive.
    pub fn FlushFileBuffers(file: HANDLE) -> BOOL;
}
