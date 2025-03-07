use crate::{Socket, winsock2::SOCKET};
use std::ops::Deref;

impl Deref for Socket {
    type Target = SOCKET;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl AsRef<SOCKET> for Socket {
    fn as_ref(&self) -> &SOCKET {
        &self.handle
    }
}
