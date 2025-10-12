use crate::{unknwn::IUnknownTrait, ComPtr};

impl<T: IUnknownTrait> Drop for ComPtr<T> {
    fn drop(&mut self) {
        self.release();
    }
}
