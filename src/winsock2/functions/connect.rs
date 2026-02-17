use crate::winsock2::{sockaddr, SOCKET};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    winsock2::{WSAGetLastError, SOCKET_ERROR},
    WSAEALREADY, WSAECONNREFUSED, WSAEINVAL, WSAEISCONN, WSAENETUNREACH, WSAETIMEDOUT,
    WSAEWOULDBLOCK,
};

#[link(name = "Ws2_32")]
unsafe extern "system" {
    /// The connect function establishes a connection to a specified socket.
    ///
    /// # Parameters
    ///  * `s` - A descriptor identifying an unconnected socket.
    ///  * `name` - A pointer to the [`sockaddr`] structure to which the connection should be
    ///             established.
    ///  * `namelen` - The length, in bytes, of the sockaddr structure pointed to by the name
    ///                parameter.
    ///
    /// # Return Value
    /// If no error occurs, connect returns zero. Otherwise, it returns [`SOCKET_ERROR`], and a
    /// specific error code can be retrieved by calling [`WSAGetLastError`].
    ///
    /// On a blocking socket, the return value indicates success or failure of the connection
    /// attempt.
    ///
    /// With a nonblocking socket, the connection attempt cannot be completed immediately. In this
    /// case, connect will return [`SOCKET_ERROR`], and [`WSAGetLastError`] will return
    /// [`WSAEWOULDBLOCK`]. In this case, there are three possible scenarios:
    ///  - Use the [`select`] function to determine the completion of the connection request by
    ///    checking to see if the socket is writable.
    ///  - If the application is using [`WSAAsyncSelect`] to indicate interest in connection
    ///    events, then the application will receive an FD_CONNECT notification indicating that the
    ///    connect operation is complete (successfully or not).
    ///  - If the application is using [`WSAEventSelect`] to indicate interest in connection
    ///    events, then the associated event object will be signaled indicating that the connect
    ///    operation is complete (successfully or not).
    ///
    /// Until the connection attempt completes on a nonblocking socket, all subsequent calls to
    /// [`connect`] on the same socket will fail with the error code [`WSAEALREADY`], and
    /// [`WSAEISCONN`] when the connection completes successfully. Due to ambiguities in version
    /// 1.1 of the Windows Sockets specification, error codes returned from connect while a
    /// connection is already pending may vary among implementations. As a result, it is not
    /// recommended that applications use multiple calls to connect to detect connection
    /// completion. If they do, they must be prepared to handle [`WSAEINVAL`] and
    /// [`WSAEWOULDBLOCK`] error values the same way that they handle [`WSAEALREADY`], to assure
    /// robust operation.
    ///
    /// If the error code returned indicates the connection attempt failed (that is,
    /// [`WSAECONNREFUSED`], [`WSAENETUNREACH`], [`WSAETIMEDOUT`]) the application can call
    /// [`connect`] again for the same socket.
    pub fn connect(s: SOCKET, name: *const sockaddr, namelen: c_int) -> c_int;
}
