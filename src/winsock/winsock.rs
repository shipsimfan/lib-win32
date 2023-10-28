use crate::{
    raw::{WSACleanup, WSAData, WSAStartup},
    Error, MAKEWORD,
};

pub struct WinSock {}

impl WinSock {
    pub fn startup() -> Result<Self, Error> {
        let mut wsa_data = WSAData::new();

        let result = unsafe { WSAStartup(MAKEWORD!(2, 2), &mut wsa_data) };
        if result != 0 {
            return Err(Error::new(result));
        }

        Ok(WinSock {})
    }
}

impl Drop for WinSock {
    fn drop(&mut self) {
        unsafe { WSACleanup() };
    }
}
