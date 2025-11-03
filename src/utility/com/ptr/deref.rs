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

impl<T2, T1: AsRef<IUnknown> + AsMut<IUnknown> + AsRef<T2> + ComInterface> AsRef<T2>
    for ComPtr<T1>
{
    fn as_ref(&self) -> &T2 {
        unsafe { self.ptr.as_ref() }.as_ref()
    }
}

impl<T2, T1: AsRef<IUnknown> + AsMut<IUnknown> + AsMut<T2> + ComInterface> AsMut<T2>
    for ComPtr<T1>
{
    fn as_mut(&mut self) -> &mut T2 {
        unsafe { self.ptr.as_mut() }.as_mut()
    }
}
