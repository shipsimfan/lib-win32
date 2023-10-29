use crate::raw::{AddrInfoW, FreeAddrInfoW};
use std::{
    marker::PhantomData,
    ops::{Deref, Index},
    ptr::NonNull,
};

pub struct AddrInfoList(NonNull<AddrInfoW>);

pub struct AddrInfoIter<'a> {
    current: Option<NonNull<AddrInfoW>>,
    phantom: PhantomData<&'a ()>,
}

impl AddrInfoList {
    pub fn new(ptr: NonNull<AddrInfoW>) -> Self {
        AddrInfoList(ptr)
    }

    pub fn get(&self, index: usize) -> Option<&AddrInfoW> {
        self.iter().skip(index).next()
    }

    pub fn iter(&self) -> AddrInfoIter {
        AddrInfoIter {
            current: Some(self.0),
            phantom: PhantomData,
        }
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

impl<'a> Iterator for AddrInfoIter<'a> {
    type Item = &'a AddrInfoW;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.current {
            Some(current) => unsafe { current.as_ref() },
            None => return None,
        };

        self.current = NonNull::new(ret.next);

        Some(ret)
    }
}
