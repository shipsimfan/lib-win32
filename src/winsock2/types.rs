use crate::{winsock2::WSADATA, HANDLE, UINT_PTR};
use std::ffi::{c_uint, c_ulong, c_void};

/// A socket group
pub type GROUP = c_uint;

/// An IPv4 transport address
#[allow(non_camel_case_types)]
pub type in_addr = c_ulong;

/// An IPv6 transport address
#[allow(non_camel_case_types)]
pub type in6_addr = [u8; 16];

/// A pointer to a [`WSADATA`] structure
pub type LPWSADATA = *mut WSADATA;

/// A pointer to a [`WSAPROTOCOL_INFOW`] structure
#[allow(non_camel_case_types)]
pub type LPWSAPROTOCOL_INFOW = *mut c_void;

/// A network socket
pub type SOCKET = UINT_PTR;

/// A Windows Socket event
pub type WSAEVENT = HANDLE;
