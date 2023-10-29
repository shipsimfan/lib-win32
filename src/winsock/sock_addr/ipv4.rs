use crate::{
    raw::{sockaddr_in, AF_INET},
    SockAddr,
};

#[repr(transparent)]
pub struct SockAddrIn(sockaddr_in);

impl SockAddr for SockAddrIn {
    const FAMILY: u16 = AF_INET as u16;
    const LENGTH: usize = std::mem::size_of::<sockaddr_in>();
}

impl Into<sockaddr_in> for SockAddrIn {
    fn into(self) -> sockaddr_in {
        self.0
    }
}

impl<'a> Into<&'a sockaddr_in> for &'a SockAddrIn {
    fn into(self) -> &'a sockaddr_in {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> Into<&'a mut sockaddr_in> for &'a mut SockAddrIn {
    fn into(self) -> &'a mut sockaddr_in {
        unsafe { std::mem::transmute(self) }
    }
}

impl From<sockaddr_in> for SockAddrIn {
    fn from(value: sockaddr_in) -> Self {
        SockAddrIn(value)
    }
}

impl<'a> From<&'a sockaddr_in> for &'a SockAddrIn {
    fn from(value: &'a sockaddr_in) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl<'a> From<&'a mut sockaddr_in> for &'a mut SockAddrIn {
    fn from(value: &'a mut sockaddr_in) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}
