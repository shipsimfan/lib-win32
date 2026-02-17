use crate::winsock2::{sockaddr, SOCKET};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::{WSAEventSelect, WSAGetLastError, INVALID_SOCKET, SOCK_STREAM};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Ws2_32")]
unsafe extern "system" {
    /// The [`accept`] function permits an incoming connection attempt on a socket.
    ///
    /// # Parameters
    ///  * `s` - A descriptor that identifies a socket that has been placed in a listening state
    ///          with the [`listen`] function. The connection is actually made with the socket that
    ///          is returned by [`accept`].
    ///  * `addr` - An optional pointer to a buffer that receives the address of the connecting
    ///             entity, as known to the communications layer. The exact format of the `addr`
    ///             parameter is determined by the address family that was established when the
    ///             socket from the [`sockaddr`] structure was created.
    ///  * `addrlen` - An optional pointer to an integer that contains the length of structure
    ///                pointed to by the `addr` parameter.
    ///
    /// # Return Value
    /// If no error occurs, [`accept`] returns a value of type [`SOCKET`] that is a descriptor for
    /// the new socket. This returned value is a handle for the socket on which the actual
    /// connection is made.
    ///
    /// Otherwise, a value of [`INVALID_SOCKET`] is returned, and a specific error code can be
    /// retrieved by calling [`WSAGetLastError`].
    ///
    /// The integer referred to by `addrlen` initially contains the amount of space pointed to by
    /// `addr`. On return it will contain the actual length in bytes of the address returned.
    ///
    /// # Remarks
    /// The [`accept`] function extracts the first connection on the queue of pending connections
    /// on socket `s`. It then creates and returns a handle to the new socket. The newly created
    /// socket is the socket that will handle the actual connection; it has the same properties as
    /// socket `s`, including the asynchronous events registered with the [`WSAAsyncSelect`] or
    /// [`WSAEventSelect`] functions.
    ///
    /// The [`accept`] function can block the caller until a connection is present if no pending
    /// connections are present on the queue, and the socket is marked as blocking. If the socket
    /// is marked as nonblocking and no pending connections are present on the queue, [`accept`]
    /// returns an error as described in the following. After the successful completion of accept
    /// returns a new socket handle, the accepted socket cannot be used to accept more connections.
    /// The original socket remains open and listens for new connection requests.
    ///
    /// The parameter `addr` is a result parameter that is filled in with the address of the
    /// connecting entity, as known to the communications layer. The exact format of the `addr`
    /// parameter is determined by the address family in which the communication is occurring. The
    /// `addrlen` is a value-result parameter; it should initially contain the amount of space
    /// pointed to by `addr`; on return it will contain the actual length (in bytes) of the address
    /// returned.
    ///
    /// The [`accept`] function is used with connection-oriented socket types such as
    /// [`SOCK_STREAM`]. If `addr` and/or `addrlen` are equal to [`null_mut`], then no information
    /// about the remote address of the accepted socket is returned.
    pub fn accept(s: SOCKET, addr: *mut sockaddr, addrlen: *mut c_int) -> SOCKET;
}
