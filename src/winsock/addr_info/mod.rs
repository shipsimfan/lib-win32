use crate::{
    raw::{sockaddr, AddrInfoW},
    SockAddr, Str,
};
use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

mod iter;
mod list;

pub use iter::*;
pub use list::*;

#[repr(transparent)]
#[derive(Clone)]
pub struct AddrInfo(AddrInfoW);

impl AddrInfo {
    pub fn new() -> Self {
        AddrInfo(AddrInfoW::new())
    }

    pub fn as_inner(&self) -> &AddrInfoW {
        &self.0
    }

    pub fn flags(&self) -> i32 {
        self.flags
    }

    pub fn family(&self) -> i32 {
        self.family
    }

    pub fn sock_type(&self) -> i32 {
        self.sock_type
    }

    pub fn protocol(&self) -> i32 {
        self.protocol
    }

    pub fn addr_len(&self) -> usize {
        self.addr_len
    }

    pub fn canon_name(&self) -> Option<&Str> {
        if self.canon_name.is_null() {
            None
        } else {
            Some(unsafe { Str::from_ptr(self.canon_name) })
        }
    }

    pub fn addr_raw(&self) -> Option<&sockaddr> {
        NonNull::new(self.addr).map(|ptr| unsafe { ptr.as_ref() })
    }

    pub fn addr_family(&self) -> Option<u16> {
        self.addr_raw().map(|addr| addr.family)
    }

    pub fn addr<S: SockAddr>(&self) -> Option<&S> {
        let family = match self.addr_family() {
            Some(family) => family,
            None => return None,
        };

        if family == S::FAMILY && self.addr_len >= std::mem::size_of::<S::Inner>() {
            Some(unsafe { &*self.addr.cast() })
        } else {
            None
        }
    }

    pub fn next(&self) -> Option<&AddrInfo> {
        NonNull::new(self.next).map(|next| unsafe { next.as_ref() }.into())
    }

    pub fn as_inner_mut(&mut self) -> &mut AddrInfoW {
        &mut self.0
    }

    pub fn flags_mut(&mut self) -> &mut i32 {
        &mut self.flags
    }

    pub fn family_mut(&mut self) -> &mut i32 {
        &mut self.family
    }

    pub fn sock_type_mut(&mut self) -> &mut i32 {
        &mut self.sock_type
    }

    pub fn addr_len_mut(&mut self) -> &mut usize {
        &mut self.addr_len
    }

    pub fn canon_name_mut(&mut self) -> Option<&mut Str> {
        if self.canon_name.is_null() {
            None
        } else {
            Some(unsafe { Str::from_ptr_mut(self.canon_name) })
        }
    }

    pub fn addr_mut<S: SockAddr>(&mut self) -> Option<&mut S> {
        let family = match self.addr_family() {
            Some(family) => family,
            None => return None,
        };

        if family == S::FAMILY && self.addr_len >= std::mem::size_of::<S::Inner>() {
            Some(unsafe { &mut *self.addr.cast() })
        } else {
            None
        }
    }

    pub fn next_mut(&mut self) -> Option<&mut AddrInfo> {
        NonNull::new(self.next).map(|mut next| unsafe { next.as_mut() }.into())
    }

    pub fn set_flags(&mut self, flags: i32) {
        self.flags = flags;
    }

    pub fn set_family(&mut self, family: i32) {
        self.family = family;
    }

    pub fn set_sock_type(&mut self, sock_type: i32) {
        self.sock_type = sock_type;
    }

    pub fn set_addr_len(&mut self, addr_len: usize) {
        self.addr_len = addr_len;
    }

    pub fn set_canon_name(&mut self, canon_name: &mut Str) {
        self.canon_name = canon_name.as_ptr() as _;
    }

    pub fn set_addr<S: SockAddr>(&mut self, addr: &mut S) {
        self.addr_len = std::mem::size_of::<S::Inner>();
        self.addr = addr as *mut _ as _;
    }

    pub fn set_next(&mut self, next: &mut AddrInfo) {
        self.next = next.as_inner_mut();
    }
}

impl<'a> Into<&'a AddrInfoW> for &'a AddrInfo {
    fn into(self) -> &'a AddrInfoW {
        unsafe { std::mem::transmute(self) }
    }
}

impl<'a> Into<&'a mut AddrInfoW> for &'a mut AddrInfo {
    fn into(self) -> &'a mut AddrInfoW {
        unsafe { std::mem::transmute(self) }
    }
}

impl Into<AddrInfoW> for AddrInfo {
    fn into(self) -> AddrInfoW {
        self.0
    }
}

impl<'a> From<&'a AddrInfoW> for &'a AddrInfo {
    fn from(value: &'a AddrInfoW) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl<'a> From<&'a mut AddrInfoW> for &'a mut AddrInfo {
    fn from(value: &'a mut AddrInfoW) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

impl From<AddrInfoW> for AddrInfo {
    fn from(value: AddrInfoW) -> Self {
        AddrInfo(value)
    }
}

impl Deref for AddrInfo {
    type Target = AddrInfoW;

    fn deref(&self) -> &Self::Target {
        self.as_inner()
    }
}

impl DerefMut for AddrInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_inner_mut()
    }
}
