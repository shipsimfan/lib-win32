use crate::{Error, Result, SocketEvent, WaitForSingleObjectEx, INFINITE, TRUE, WAIT_FAILED};

impl SocketEvent {
    /// Wait until the event is signalled, returning `true` if the event was signalled, or `false`
    /// if a different reason caused the wait to end..
    pub fn wait(&self) -> Result<bool> {
        match unsafe { WaitForSingleObjectEx(self.handle, INFINITE, TRUE) } {
            0 => Ok(true),
            WAIT_FAILED => Err(Error::get_last_error()),
            _ => Ok(false),
        }
    }
}
