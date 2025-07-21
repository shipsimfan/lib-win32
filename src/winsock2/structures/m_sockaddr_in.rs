use crate::winsock2::{in_addr, AF_INET};
use std::ffi::{c_char, c_short, c_ushort};

/// An IPv4 socket address
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct sockaddr_in {
    /// [`AF_INET`]
    pub family: c_short,

    /// The port
    pub port: c_ushort,

    /// The IPv4 address
    pub addr: in_addr,

    /// Padding
    pub zero: [c_char; 8],
}

impl Default for sockaddr_in {
    fn default() -> Self {
        sockaddr_in {
            family: AF_INET as _,
            port: 0,
            addr: 0,
            zero: [0; 8],
        }
    }
}
