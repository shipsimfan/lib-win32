use crate::{winsock2::WSADATA, UINT_PTR};
use std::ffi::c_ulong;

/// An IPv4 transport address
#[allow(non_camel_case_types)]
pub type in_addr = c_ulong;

/// An IPv6 transport address
#[allow(non_camel_case_types)]
pub type in6_addr = [u8; 16];

/// A pointer to a [`WSADATA`] structure
pub type LPWSADATA = *mut WSADATA;

/// A network socket
pub type SOCKET = UINT_PTR;
