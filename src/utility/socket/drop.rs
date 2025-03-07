use crate::{winsock2::closesocket, Socket};

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { closesocket(self.handle) };
    }
}
