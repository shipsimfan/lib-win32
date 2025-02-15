use crate::{ComInterface, ComPtr};

impl<T: ComInterface> Drop for ComPtr<T> {
    fn drop(&mut self) {
        self.release();
    }
}
