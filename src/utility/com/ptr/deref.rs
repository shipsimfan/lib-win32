use crate::{unknwn::IUnknown, ComInterface, ComPtr};
use std::ops::{Deref, DerefMut};

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> Deref for ComPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> DerefMut for ComPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> AsRef<T> for ComPtr<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> AsMut<T> for ComPtr<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> AsRef<IUnknown> for ComPtr<T> {
    fn as_ref(&self) -> &IUnknown {
        unsafe { self.ptr.as_ref() }.as_ref()
    }
}

impl<T: AsRef<IUnknown> + AsMut<IUnknown> + ComInterface> AsMut<IUnknown> for ComPtr<T> {
    fn as_mut(&mut self) -> &mut IUnknown {
        unsafe { self.ptr.as_mut() }.as_mut()
    }
}
