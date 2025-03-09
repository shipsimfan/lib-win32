use crate::{
    winsock2::{socket, INVALID_SOCKET, SOCKET},
    Error, Result, Socket,
};
use std::ffi::c_int;

impl Socket {
    /// Creates a new [`Socket`]
    pub fn new(family: c_int, r#type: c_int, protocol: c_int) -> Result<Self> {
        let handle = unsafe { socket(family, r#type, protocol) };
        if handle != INVALID_SOCKET {
            Ok(Socket { handle })
        } else {
            Err(Error::wsa_get_last_error())
        }
    }

    /// Creates a new [`Socket`] from a raw `handle`
    pub unsafe fn from_raw(handle: SOCKET) -> Self {
        Socket { handle }
    }
}
