use crate::{unknwn::IUnknownTrait, ComPtr};
use std::ptr::NonNull;

impl<T: IUnknownTrait> ComPtr<T> {
    /// Creates a new [`ComPtr`], panicking if `ptr` is null
    pub fn new(ptr: *mut T) -> Self {
        ComPtr {
            ptr: NonNull::new(ptr).unwrap(),
        }
    }
}
