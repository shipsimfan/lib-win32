use crate::{unknwn::IUnknown, ComInterface, ComPtr};

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> Drop for ComPtr<T> {
    fn drop(&mut self) {
        AsMut::<IUnknown>::as_mut(self).release();
    }
}
