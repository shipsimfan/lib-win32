use crate::{winsock2::WSAEVENT, SocketEvent};
use std::ops::Deref;

impl Deref for SocketEvent {
    type Target = WSAEVENT;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl AsRef<WSAEVENT> for SocketEvent {
    fn as_ref(&self) -> &WSAEVENT {
        &self.handle
    }
}
