use crate::winsock2::{in6_addr, AF_INET6};
use std::ffi::{c_short, c_ulong, c_ushort};

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

impl Default for sockaddr_in6 {
    fn default() -> Self {
        sockaddr_in6 {
            family: AF_INET6 as _,
            port: 0,
            flowinfo: 0,
            addr: [0; 16],
            scope_id: 0,
        }
    }
}
