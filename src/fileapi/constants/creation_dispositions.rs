use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{ERROR_ALREADY_EXISTS, ERROR_FILE_EXISTS, ERROR_FILE_NOT_FOUND, GENERIC_WRITE};

/// Creates a new file, always.
///
/// If the specified file exists and is writable, the function truncates the file, the function
/// succeeds, and last-error code is set to [`ERROR_ALREADY_EXISTS`] (183).
///
/// If the specified file does not exist and is a valid path, a new file is created, the function
/// succeeds, and the last-error code is set to zero.
pub const CREATE_ALWAYS: DWORD = 2;

/// Creates a new
///
/// file, only if it does not already exist.If the specified file exists, the function fails and
/// the last-error code is set to [`ERROR_FILE_EXISTS`] (80).
///
/// If the specified file does not exist and is a valid path to a writable location, a new file is
/// created.
pub const CREATE_NEW: DWORD = 1;

/// Opens a file, always.
///
/// If the specified file exists, the function succeeds and the last-error code is set to
/// [`ERROR_ALREADY_EXISTS`] (183).
///
/// If the specified file does not exist and is a valid path to a writable location, the function
/// creates a file and the last-error code is set to zero.
pub const OPEN_ALWAYS: DWORD = 4;

/// Opens a file or device, only if it exists.
///
/// If the specified file or device does not exist, the function fails and the last-error code is
/// set to [`ERROR_FILE_NOT_FOUND`] (2).
pub const OPEN_EXISTING: DWORD = 3;

/// Opens a file and truncates it so that its size is zero bytes, only if it exists.
///
/// If the specified file does not exist, the function fails and the last-error code is set to
/// [`ERROR_FILE_NOT_FOUND`] (2).
///
/// The calling process must open the file with the [`GENERIC_WRITE`] bit set as part of the
/// `desired_access` parameter.
pub const TRUNCATE_EXISTING: DWORD = 5;
