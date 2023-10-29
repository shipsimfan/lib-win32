use crate::{
    raw::{sockaddr_in, AF_INET},
    InAddr, SockAddr,
};
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

#[repr(transparent)]
#[derive(Clone)]
pub struct SockAddrIn(sockaddr_in);

impl SockAddrIn {
    pub fn family(&self) -> i16 {
        self.family
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn addr(&self) -> InAddr {
        self.addr.into()
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_addr(&mut self, addr: InAddr) {
        self.addr = addr.into();
    }
}

impl SockAddr for SockAddrIn {
    const FAMILY: u16 = AF_INET as u16;

    type Inner = sockaddr_in;
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

impl Deref for SockAddrIn {
    type Target = sockaddr_in;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SockAddrIn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for SockAddrIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.addr(), self.port)
    }
}

impl Debug for SockAddrIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
