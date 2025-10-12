use crate::{unknwn::IUnknownTrait, ComPtr};

impl<T: IUnknownTrait> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        let mut ptr = ComPtr::new(self.ptr.as_ptr());
        ptr.add_ref();
        ptr
    }
}
