use crate::{winsock2::WSACloseEvent, SocketEvent};

impl Drop for SocketEvent {
    fn drop(&mut self) {
        unsafe { WSACloseEvent(self.handle) };
    }
}
