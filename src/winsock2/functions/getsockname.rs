use crate::winsock2::{sockaddr, SOCKET};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::{WSAGetLastError, SOCKET_ERROR};

#[link(name = "Ws2_32")]
extern "system" {
    /// The [`getsockname`] function retrieves the local name for a socket.
    ///
    /// # Parameters
    ///  * `s` - Descriptor identifying a socket.
    ///  * `name` - Pointer to a [`sockaddr`] structure that receives the address (name) of the
    ///             socket.
    ///  * `name_len` - Size of the `name` buffer, in bytes.
    ///
    /// # Return Value
    /// If no error occurs, [`getsockname`] returns zero. Otherwise, a value of [`SOCKET_ERROR`] is
    /// returned, and a specific error code can be retrieved by calling [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`getsockname`] function retrieves the current name for the specified socket descriptor
    /// in name. It is used on the bound or connected socket specified by the s parameter. The
    /// local association is returned. This call is especially useful when a connect call has been
    /// made without doing a bind first; the [`getsockname`] function provides the only way to
    /// determine the local association that has been set by the system.
    ///
    /// On call, the `name_len` parameter contains the size of the `name` buffer, in bytes. On
    /// return, the `name_len` parameter contains the actual size in bytes of the `name` parameter.
    ///
    /// The [`getsockname`] function does not always return information about the host address when
    /// the socket has been bound to an unspecified address, unless the socket has been connected
    /// with connect or accept (for example, using [`ADDR_ANY`]). A Windows Sockets application
    /// must not assume that the address will be specified unless the socket is connected. The
    /// address that will be used for the socket is unknown unless the socket is connected when
    /// used in a multihomed host. If the socket is using a connectionless protocol, the address
    /// may not be available until I/O occurs on the socket.
    pub fn getsockname(s: SOCKET, name: *mut sockaddr, name_len: *mut c_int) -> c_int;
}
