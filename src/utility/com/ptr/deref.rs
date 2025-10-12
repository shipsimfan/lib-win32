use crate::{unknwn::IUnknownTrait, ComPtr};
use std::ops::{Deref, DerefMut};

impl<T: IUnknownTrait> Deref for ComPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: IUnknownTrait> DerefMut for ComPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: IUnknownTrait> AsRef<T> for ComPtr<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: IUnknownTrait> AsMut<T> for ComPtr<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}
