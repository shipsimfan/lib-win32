use crate::{COMInterface, COMPtr};

impl<T: COMInterface> Clone for COMPtr<T> {
    fn clone(&self) -> Self {
        let mut ptr = COMPtr::new(self.ptr.as_ptr());
        ptr.add_ref();
        ptr
    }
}
