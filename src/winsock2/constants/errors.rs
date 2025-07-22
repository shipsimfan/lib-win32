use crate::{winsock2::SOCKET, DWORD};
use std::ffi::c_int;

/// An invalid socket
pub const INVALID_SOCKET: SOCKET = !0;

/// An error occurred with a socket
pub const SOCKET_ERROR: c_int = -1;

/// Overlapped operations will complete later.
///
/// The application has initiated an overlapped operation that cannot be completed immediately. A
/// completion indication will be given later when the operation has been completed.
pub const WSA_IO_PENDING: DWORD = 997;
