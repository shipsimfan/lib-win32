use crate::{
    raw::{AddrInfoW, GetAddrInfoW, WSACleanup, WSAData, WSASocketW, WSAStartup, INVALID_SOCKET},
    AddrInfoList, Error, Socket, Str, MAKEWORD,
};
use std::ptr::{null_mut, NonNull};

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

    pub fn get_addr_info(
        &self,
        node_name: Option<&Str>,
        service_name: Option<&Str>,
        flags: i32,
        family: i32,
        sock_type: i32,
        protocol: i32,
    ) -> Result<AddrInfoList, Error> {
        let mut hints = AddrInfoW::new();
        hints.flags = flags;
        hints.family = family;
        hints.sock_type = sock_type;
        hints.protocol = protocol;

        let mut ret = null_mut();
        let result = unsafe {
            GetAddrInfoW(
                node_name.map(|ptr| ptr.as_ptr()).unwrap_or(null_mut()),
                service_name.map(|ptr| ptr.as_ptr()).unwrap_or(null_mut()),
                &hints,
                &mut ret,
            )
        };
        if result != 0 || ret == null_mut() {
            return Err(Error::new(result));
        }

        // SAFTEY: `ret` is check for null above
        Ok(AddrInfoList::new(unsafe { NonNull::new_unchecked(ret) }))
    }

    pub fn socket(&self, family: i32, sock_type: i32, protocol: i32) -> Result<Socket, Error> {
        let socket = unsafe { WSASocketW(family, sock_type, protocol, null_mut(), 0, 0) };
        if socket == INVALID_SOCKET {
            return Err(Error::wsa_last_error());
        }

        Ok(Socket::new(socket))
    }
}

impl Drop for WinSock {
    fn drop(&mut self) {
        unsafe { WSACleanup() };
    }
}
