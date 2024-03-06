use crate::winsock2::SOCKET;
use std::ffi::{c_int, c_long, c_ulong};

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::{WSAGetLastError, SOCKET_ERROR};

#[link(name = "Ws2_32")]
extern "system" {
    /// The [`ioctlsocket`] function controls the I/O mode of a socket.
    ///
    /// # Parameters
    ///  * `s` - A descriptor identifying a socket.
    ///  * `cmd` - A command to perform on the socket `s`.
    ///  * `argp` - A pointer to a parameter for `cmd`.
    ///
    /// # Return Value
    /// Upon successful completion, the [`ioctlsocket`] returns zero. Otherwise, a value of
    /// [`SOCKET_ERROR`] is returned, and a specific error code can be retrieved by calling
    /// [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`ioctlsocket`] function can be used on any socket in any state. It is used to set or
    /// retrieve some operating parameters associated with the socket, independent of the protocol
    /// and communications subsystem.
    ///
    /// The [`WSAIoctl`] function is used to set or retrieve operating parameters associated with
    /// the socket, the transport protocol, or the communications subsystem.
    ///
    /// The [`WSAIoctl`] function is more powerful than the ioctlsocket function and supports a
    /// large number of possible values for the operating parameters to set or retrieve.
    pub fn ioctlsocket(s: SOCKET, cmd: c_long, argp: *mut c_ulong) -> c_int;
}
