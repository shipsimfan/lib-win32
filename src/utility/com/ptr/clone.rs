use crate::{ComInterface, ComPtr};

impl<T: ComInterface> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        let mut ptr = ComPtr::new(self.ptr.as_ptr());
        ptr.add_ref();
        ptr
    }
}
