use crate::{COMInterface, COMPtr};
use std::ptr::NonNull;

impl<T: COMInterface> COMPtr<T> {
    /// Creates a new [`COMPtr`], panicking if `ptr` is null
    pub fn new(ptr: *mut T) -> Self {
        COMPtr {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }
}
