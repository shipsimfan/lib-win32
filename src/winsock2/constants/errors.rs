use crate::winsock2::SOCKET;
use std::ffi::c_int;

/// An invalid socket
pub const INVALID_SOCKET: SOCKET = !0;

/// An error occurred with a socket
pub const SOCKET_ERROR: c_int = -1;
