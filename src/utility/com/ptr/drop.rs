use crate::{COMInterface, COMPtr};

impl<T: COMInterface> Drop for COMPtr<T> {
    fn drop(&mut self) {
        self.release();
    }
}
