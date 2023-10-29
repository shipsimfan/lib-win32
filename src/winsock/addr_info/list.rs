use crate::{
    raw::{AddrInfoW, FreeAddrInfoW},
    AddrInfo, AddrInfoIter,
};
use std::{
    ops::{Deref, Index},
    ptr::NonNull,
};

pub struct AddrInfoList(NonNull<AddrInfoW>);

impl AddrInfoList {
    pub fn new(ptr: NonNull<AddrInfoW>) -> Self {
        AddrInfoList(ptr)
    }

    pub fn get(&self, index: usize) -> Option<&AddrInfo> {
        self.iter().skip(index).next().map(|addr| addr.into())
    }

    pub fn iter(&self) -> AddrInfoIter {
        AddrInfoIter::new(self.0)
    }
}

impl Deref for AddrInfoList {
    type Target = AddrInfoW;

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.as_ref() }
    }
}

impl Index<usize> for AddrInfoList {
    type Output = AddrInfoW;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl Drop for AddrInfoList {
    fn drop(&mut self) {
        unsafe { FreeAddrInfoW(self.0.as_ptr()) }
    }
}
