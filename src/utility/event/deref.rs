use crate::{Event, HANDLE};
use std::ops::Deref;

impl Deref for Event {
    type Target = HANDLE;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl AsRef<HANDLE> for Event {
    fn as_ref(&self) -> &HANDLE {
        &self.handle
    }
}
