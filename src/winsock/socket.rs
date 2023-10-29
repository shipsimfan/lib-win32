use crate::raw::{self, closesocket};

pub struct Socket {
    socket: raw::Socket,
}

impl Socket {
    pub fn new(socket: raw::Socket) -> Self {
        Socket { socket }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { closesocket(self.socket) };
    }
}
