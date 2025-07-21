use crate::{
    try_wsa_get_last_error,
    winsock2::{listen, SOMAXCONN},
    Result, Socket,
};

impl Socket {
    /// Set the socket into the "LISTEN" state to begin accepting clients
    pub fn listen(&mut self, backlog: Option<usize>) -> Result<()> {
        try_wsa_get_last_error!(listen(self.handle, backlog.unwrap_or(SOMAXCONN as _) as _))
            .map(|_| ())
    }
}
