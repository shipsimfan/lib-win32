use crate::{unknwn::IUnknown, ComInterface, ComPtr};

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> Clone for ComPtr<T> {
    fn clone(&self) -> Self {
        let mut ptr = ComPtr::new(self.ptr.as_ptr());
        AsMut::<IUnknown>::as_mut(&mut ptr).add_ref();
        ptr
    }
}
