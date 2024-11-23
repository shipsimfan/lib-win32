use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateFile, CREATE_ALWAYS, FILE_SHARE_DELETE};

/// The file is being opened or created for a backup or restore operation. The system ensures that
/// the calling process overrides file security checks when the process has [`SE_BACKUP_NAME`] and
/// [`SE_RESTORE_NAME`] privileges.
///
/// You must set this flag to obtain a handle to a directory. A directory handle can be passed to
/// some functions instead of a file handle.
pub const FILE_FLAG_BACKUP_SEMANTICS: DWORD = 0x02000000;

/// The file is to be deleted immediately after all of its handles are closed, which includes the
/// specified handle and any other open or duplicated handles.
///
/// If there are existing open handles to a file, the call fails unless they were all opened with
/// the [`FILE_SHARE_DELETE`] share mode.
///
/// Subsequent open requests for the file fail, unless the [`FILE_SHARE_DELETE`] share mode is
/// specified.
pub const FILE_FLAG_DELETE_ON_CLOSE: DWORD = 0x04000000;

/// The file or device is being opened with no system caching for data reads and writes. This flag
/// does not affect hard disk caching or memory mapped files.
///
/// There are strict requirements for successfully working with files opened with [`CreateFile`]
/// using the [`FILE_FLAG_NO_BUFFERING`] flag.
pub const FILE_FLAG_NO_BUFFERING: DWORD = 0x20000000;

/// The file data is requested, but it should continue to be located in remote storage. It should
/// not be transported back to local storage. This flag is for use by remote storage systems.
pub const FILE_FLAG_OPEN_NO_RECALL: DWORD = 0x00100000;

/// Normal reparse point processing will not occur; [`CreateFile`] will attempt to open the reparse
/// point. When a file is opened, a file handle is returned, whether or not the filter that
/// controls the reparse point is operational.
///
/// This flag cannot be used with the [`CREATE_ALWAYS`] flag.
///
/// If the file is not a reparse point, then this flag is ignored.
pub const FILE_FLAG_OPEN_REPARSE_POINT: DWORD = 0x00200000;

/// The file or device is being opened or created for asynchronous I/O.
///
/// When subsequent I/O operations are completed on this handle, the event specified in the
/// [`OVERLAPPED`] structure will be set to the signaled state.
///
/// If this flag is specified, the file can be used for simultaneous read and write operations.
///
/// If this flag is not specified, then I/O operations are serialized, even if the calls to the
/// read and write functions specify an [`OVERLAPPED`] structure.
pub const FILE_FLAG_OVERLAPPED: DWORD = 0x40000000;

/// Access will occur according to POSIX rules. This includes allowing multiple files with names,
/// differing only in case, for file systems that support that naming. Use care when using this
/// option, because files created with this flag may not be accessible by applications that are
/// written for MS-DOS or 16-bit Windows.
pub const FILE_FLAG_POSIX_SEMANTICS: DWORD = 0x01000000;

/// Access is intended to be random. The system can use this as a hint to optimize file caching.
///
/// This flag has no effect if the file system does not support cached I/O and
/// [`FILE_FLAG_NO_BUFFERING`].
pub const FILE_FLAG_RANDOM_ACCESS: DWORD = 0x10000000;

/// The file or device is being opened with session awareness. If this flag is not specified, then
/// per-session devices (such as a device using RemoteFX USB Redirection) cannot be opened by
/// processes running in session 0. This flag has no effect for callers not in session 0. This flag
/// is supported only on server editions of Windows.
pub const FILE_FLAG_SESSION_AWARE: DWORD = 0x00800000;

/// Access is intended to be sequential from beginning to end. The system can use this as a hint to
/// optimize file caching.
///
/// This flag should not be used if read-behind (that is, reverse scans) will be used.
///
/// This flag has no effect if the file system does not support cached I/O and
/// [`FILE_FLAG_NO_BUFFERING`].
pub const FILE_FLAG_SEQUENTIAL_SCAN: DWORD = 0x08000000;

/// Write operations will not go through any intermediate cache, they will go directly to disk.
pub const FILE_FLAG_WRITE_THROUGH: DWORD = 0x80000000;
