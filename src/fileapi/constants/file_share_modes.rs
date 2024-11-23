use crate::DWORD;

/// Enables subsequent open operations on a file or device to request delete access. Otherwise, no
/// process can open the file or device if it requests delete access.
///
/// If this flag is not specified, but the file or device has been opened for delete access, the
/// function fails.
pub const FILE_SHARE_DELETE: DWORD = 0x00000004;

/// Enables subsequent open operations on a file or device to request read access. Otherwise, no
/// process can open the file or device if it requests read access.
///
/// If this flag is not specified, but the file or device has been opened for read access, the
/// function fails.
pub const FILE_SHARE_READ: DWORD = 0x00000001;

/// Enables subsequent open operations on a file or device to request write access. Otherwise, no
/// process can open the file or device if it requests write access.
///
/// If this flag is not specified, but the file or device has been opened for write access or has a
/// file mapping with write access, the function fails.
pub const FILE_SHARE_WRITE: DWORD = 0x00000002;
