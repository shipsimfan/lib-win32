use crate::raw::WSACleanup;

pub struct WinSock {}

impl WinSock {
    pub fn startup() -> Result<Self, Win32Error> {
        todo!()
    }
}

impl Drop for WinSock {
    fn drop(&mut self) {
        unsafe { WSACleanup() };
    }
}
