use crate::{Event, HANDLE};

impl Event {
    /// Get the underlying event handle
    pub const fn handle(&self) -> HANDLE {
        self.handle
    }
}
