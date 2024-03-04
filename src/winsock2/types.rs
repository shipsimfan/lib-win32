use crate::UINT_PTR;
use std::ffi::{c_ulong, c_void};

/// An IPv4 transport address
#[allow(non_camel_case_types)]
pub type in_addr = c_ulong;

/// An IPv6 transport address
#[allow(non_camel_case_types)]
pub type in6_addr = [u8; 16];

/// A pointer to a [`WSADATA`] structure
pub type LPWSADATA = *mut c_void;

/// A network socket
pub type SOCKET = UINT_PTR;
