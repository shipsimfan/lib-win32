use crate::{raw::WSACleanup, Error};

pub struct WinSock {}

impl WinSock {
    pub fn startup() -> Result<Self, Error> {
        todo!()
    }
}

impl Drop for WinSock {
    fn drop(&mut self) {
        unsafe { WSACleanup() };
    }
}
