use crate::{ComInterface, ComPtr};
use std::ptr::NonNull;

impl<T: ComInterface> ComPtr<T> {
    /// Creates a new [`COMPtr`], panicking if `ptr` is null
    pub fn new(ptr: *mut T) -> Self {
        ComPtr {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }
}
