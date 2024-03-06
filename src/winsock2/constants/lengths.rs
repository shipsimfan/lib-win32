use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::listen;

/// The maximum size of a description for a WSA DLL
pub const WSADESCRIPTION_LEN: usize = 256;

/// The maximum size of a status for a WSA DLL
pub const WSASYS_STATUS_LEN: usize = 128;

/// Maximum queue length specifiable by [`listen`]
pub const SOMAXCONN: c_int = 0x7FFFFFFF;
