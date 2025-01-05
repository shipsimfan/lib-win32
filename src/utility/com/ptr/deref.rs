use crate::{COMInterface, COMPtr};
use std::ops::{Deref, DerefMut};

impl<T: COMInterface> Deref for COMPtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: COMInterface> DerefMut for COMPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T: COMInterface> AsRef<T> for COMPtr<T> {
    fn as_ref(&self) -> &T {
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: COMInterface> AsMut<T> for COMPtr<T> {
    fn as_mut(&mut self) -> &mut T {
        unsafe { self.ptr.as_mut() }
    }
}
