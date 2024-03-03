use crate::winsock2::in6_addr;
use std::ffi::{c_short, c_ulong, c_ushort};

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::AF_INET6;

/// An IPv6 socket address
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct sockaddr_in6 {
    /// [`AF_INET6`]
    pub family: c_short,

    /// The port
    pub port: c_ushort,

    /// IPv6 flow information
    pub flowinfo: c_ulong,

    /// The IPv6 address
    pub addr: in6_addr,

    /// Scope ID
    pub scope_id: c_ulong,
}
