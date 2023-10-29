use crate::{raw::AddrInfoW, AddrInfo};
use std::{marker::PhantomData, ptr::NonNull};

pub struct AddrInfoIter<'a> {
    current: Option<NonNull<AddrInfoW>>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> AddrInfoIter<'a> {
    pub(super) fn new(start: NonNull<AddrInfoW>) -> Self {
        AddrInfoIter {
            current: Some(start),
            phantom: PhantomData,
        }
    }
}

impl<'a> Iterator for AddrInfoIter<'a> {
    type Item = &'a AddrInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.current {
            Some(current) => unsafe { current.as_ref() },
            None => return None,
        };

        self.current = NonNull::new(ret.next);

        Some(ret.into())
    }
}
