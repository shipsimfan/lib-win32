use crate::{CloseHandle, Event};

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.handle) };
    }
}
