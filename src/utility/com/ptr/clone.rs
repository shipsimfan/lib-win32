use crate::{ComInterface, ComPtr, unknwn::IUnknown};

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        let mut ptr = ComPtr::new(self.ptr.as_ptr());
        unsafe { AsMut::<IUnknown>::as_mut(&mut ptr).add_ref() };
        ptr
    }
}
