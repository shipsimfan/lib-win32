use crate::{
    winsock2::{WSACreateEvent, WSAEVENT, WSA_INVALID_EVENT},
    Error, Result, SocketEvent,
};

impl SocketEvent {
    /// Creates a new [`SocketEvent`]
    pub fn new() -> Result<Self> {
        match unsafe { WSACreateEvent() } {
            WSA_INVALID_EVENT => Err(Error::wsa_get_last_error()),
            handle => Ok(SocketEvent { handle }),
        }
    }

    /// Creates a new [`SocketEvent`] with default settings
    pub fn default() -> Result<Self> {
        SocketEvent::new()
    }

    /// Creates a new [`SocketEvent`] from a raw `handle`
    pub unsafe fn from_raw(handle: WSAEVENT) -> SocketEvent {
        SocketEvent { handle }
    }
}
