use crate::{winsock2::SOCKET, Socket};

impl Socket {
    /// Get the underlying socket handle
    pub fn handle(&self) -> SOCKET {
        self.handle
    }
}
