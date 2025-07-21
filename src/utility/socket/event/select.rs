use crate::{try_wsa_get_last_error, winsock2::WSAEventSelect, Result, Socket, SocketEvent};
use std::ffi::c_long;

impl SocketEvent {
    /// Associates a [`Socket`] to a [`SocketEvent`] with certain `events`
    pub fn select(&mut self, socket: &Socket, events: c_long) -> Result<()> {
        try_wsa_get_last_error!(WSAEventSelect(socket.handle(), self.handle, events)).map(|_| ())
    }
}
