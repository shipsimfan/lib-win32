use crate::{try_get_last_error, Event, ResetEvent, Result, SetEvent};

impl Event {
    /// Signals a thread waiting on this event to wake up
    pub fn set(&self) -> Result<()> {
        try_get_last_error!(SetEvent(self.handle)).map(|_| ())
    }

    /// Resets the event to an unsignalled state
    pub fn reset(&self) -> Result<()> {
        try_get_last_error!(ResetEvent(self.handle)).map(|_| ())
    }
}
