use crate::raw::Socket;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{WSAGetLastError, SOCKET_ERROR};

#[link(name = "Ws2_32")]
extern "C" {
    /// # closesocket function (winsock.h)
    ///
    /// The [`closesocket`] function closes an existing socket.
    ///
    /// ## Parameters
    /// `s`\
    /// A descriptor identifying the socket to close.
    ///
    /// ## Return Value
    /// If no error occurs, [`closesocket`] returns zero. Otherwise, a value of
    /// [`SOCKET_ERROR`] is returned, and a specific error code can be
    /// retrieved by calling [`WSAGetLastError`].
    pub fn closesocket(s: Socket) -> c_int;
}
