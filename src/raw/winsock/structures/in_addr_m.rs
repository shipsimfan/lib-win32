use std::ffi::{c_uchar, c_ulong, c_ushort};

/// # in_addr structure (winsock2.h)
///
/// The **in_addr** structure represents an IPv4 Internet address.
///
/// ## Members
/// `un`
///
/// `un.un_b`\
/// An IPv4 address formatted as four [`c_uchar`]s.\
/// `un.un_b.b1`\
/// `un.un_b.b2`\
/// `un.un_b.b3`\
/// `un.un_b.b4`
///
/// `un.un_w`\
/// An IPv4 address formatted as two [`c_ushort`]s.\
/// `un.un_w.w1`\
/// `un.un_w.w2`
///
/// `un.addr`\
/// An IPv4 address formatted as a [`c_ulong`].
///
/// ## Remarks
/// The **in_addr** structure is used with IPv4 addresses.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct in_addr {
    pub un: in_addr_union,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub union in_addr_union {
    pub un_b: in_addr_b,
    pub un_w: in_addr_w,
    pub addr: c_ulong,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr_b {
    pub b1: c_uchar,
    pub b2: c_uchar,
    pub b3: c_uchar,
    pub b4: c_uchar,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct in_addr_w {
    pub w1: c_ushort,
    pub w2: c_ushort,
}

impl in_addr {
    pub fn new() -> Self {
        in_addr {
            un: in_addr_union {
                un_b: in_addr_b {
                    b1: 0,
                    b2: 0,
                    b3: 0,
                    b4: 0,
                },
            },
        }
    }
}

impl PartialEq for in_addr_union {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.addr.eq(&other.addr) }
    }
}

impl Eq for in_addr_union {}

impl PartialOrd for in_addr_union {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        unsafe { self.addr.partial_cmp(&other.addr) }
    }
}

impl Ord for in_addr_union {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        unsafe { self.addr.cmp(&other.addr) }
    }
}
