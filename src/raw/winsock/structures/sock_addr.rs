//! # sockaddr
//!
//! The [`sockaddr`] structure varies depending on the protocol selected.
//! Except for the sin*_family parameter, sockaddr contents are expressed in
//! network byte order.
//!
//! Winsock functions using [`sockaddr`] are not strictly interpreted to be
//! pointers to a [`sockaddr`] structure. The structure is interpreted
//! differently in the context of different address families. The only
//! requirements are that the first [`c_ushort`] is the address family and the
//! total size of the memory buffer in bytes is `namelen`.
//!
//! The [`sockaddr`] structure and [`sockaddr_in`] structures below are used
//! with IPv4. Other protocols use similar structures.

use crate::raw::in_addr;
use std::ffi::{c_char, c_short, c_ushort};

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone)]
pub struct sockaddr {
    pub family: c_ushort,
    pub data: [c_char; 14],
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone)]
pub struct sockaddr_in {
    pub family: c_short,
    pub port: c_ushort,
    pub addr: in_addr,
    pub zero: [c_char; 8],
}

impl sockaddr {
    pub fn new() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
