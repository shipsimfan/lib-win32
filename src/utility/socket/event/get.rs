use crate::{winsock2::WSAEVENT, SocketEvent};

impl SocketEvent {
    /// Get the underlying event handle
    pub const fn handle(&self) -> WSAEVENT {
        self.handle
    }
}
