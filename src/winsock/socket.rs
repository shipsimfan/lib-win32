use crate::{
    raw::{self, bind, closesocket},
    Error, SockAddr,
};
use std::ffi::c_int;

pub struct Socket {
    socket: raw::Socket,
}

impl Socket {
    pub fn new(socket: raw::Socket) -> Self {
        Socket { socket }
    }

    pub fn bind<S: SockAddr>(&self, addr: &S) -> Result<(), Error> {
        let result = unsafe {
            bind(
                self.socket,
                addr as *const _ as _,
                std::mem::size_of::<S::Inner>() as c_int,
            )
        };

        if result != 0 {
            Err(Error::wsa_last_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { closesocket(self.socket) };
    }
}
