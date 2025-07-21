use crate::{try_wsa_get_last_error, winsock2::bind, Result, Socket, SocketAddress};

impl Socket {
    /// Bind a socket to a name
    pub fn bind(&mut self, address: &SocketAddress) -> Result<()> {
        try_wsa_get_last_error!(bind(self.handle, address.as_ptr(), address.len() as _)).map(|_| ())
    }
}
