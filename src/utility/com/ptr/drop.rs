use crate::{ComInterface, ComPtr, unknwn::IUnknown};

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> Drop for ComPtr<T> {
    fn drop(&mut self) {
        unsafe { AsMut::<IUnknown>::as_mut(self).release() };
    }
}
