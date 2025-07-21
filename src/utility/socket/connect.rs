use crate::{try_wsa_get_last_error, winsock2::connect, Result, Socket, SocketAddress};

impl Socket {
    /// Initiate a connection on a socket
    pub fn connect(&mut self, address: &SocketAddress) -> Result<()> {
        try_wsa_get_last_error!(connect(self.handle, address.as_ptr(), address.len() as _))
            .map(|_| ())
    }
}
