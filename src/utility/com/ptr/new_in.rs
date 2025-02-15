use crate::{ComInterface, ComPtr, Error};
use std::ptr::null_mut;

impl<T: ComInterface> ComPtr<T> {
    /// Creates a new [`ComPtr`] by calling `f` with an uninitialized pointer which it will fill
    pub fn new_in<R, F: FnOnce(*mut *mut T) -> Result<R, Error>>(f: F) -> Result<ComPtr<T>, Error> {
        let mut ptr = null_mut();
        f(&mut ptr).map(|_| ComPtr::new(ptr))
    }
}
