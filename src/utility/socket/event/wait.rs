use crate::{
    Error, Result, SocketEvent, WaitForSingleObjectEx, FALSE, INFINITE, TRUE, WAIT_FAILED,
};
use std::time::Duration;

impl SocketEvent {
    /// Wait until the event is signalled, returning `true` if the event was signalled, or `false`
    /// if a different reason caused the wait to end, such as a timeout or async I/O callback
    pub fn wait(&self, timeout: Option<Duration>, alertable: bool) -> Result<bool> {
        match unsafe {
            WaitForSingleObjectEx(
                self.handle,
                timeout
                    .map(|timeout| timeout.as_millis() as _)
                    .unwrap_or(INFINITE),
                if alertable { TRUE } else { FALSE },
            )
        } {
            0 => Ok(true),
            WAIT_FAILED => Err(Error::get_last_error()),
            _ => Ok(false),
        }
    }
}
