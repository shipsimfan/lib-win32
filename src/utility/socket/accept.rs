use crate::{
    winsock2::{accept, sockaddr_in6, AF_INET, AF_INET6, INVALID_SOCKET},
    Error, Result, Socket, SocketAddress,
};

impl Socket {
    /// Accept a connection on a socket
    pub fn accept(&self) -> Result<(Self, Option<SocketAddress>)> {
        let mut address = sockaddr_in6::default();
        let mut len = std::mem::size_of::<sockaddr_in6>() as _;

        let handle = unsafe { accept(self.handle, &mut address as *mut _ as _, &mut len) };
        if handle == INVALID_SOCKET {
            return Err(Error::wsa_get_last_error());
        }

        let address = match address.family as _ {
            AF_INET => {
                let sockaddr_in = unsafe { std::mem::transmute_copy(&address) };
                Some(SocketAddress::V4(sockaddr_in))
            }
            AF_INET6 => Some(SocketAddress::V6(address)),
            _ => None,
        };

        Ok((unsafe { Socket::from_raw(handle) }, address))
    }
}
