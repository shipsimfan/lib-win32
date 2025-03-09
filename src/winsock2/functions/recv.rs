use crate::winsock2::SOCKET;
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    winsock2::{
        accept, bind, connect, WSAEventSelect, WSAGetLastError, SOCKET_ERROR, SOCK_DGRAM,
        SOCK_STREAM,
    },
    WSAECONNRESET, WSAEMSGSIZE, WSAEWOULDBLOCK,
};

#[link(name = "Ws2_32")]
extern "system" {
    /// The [`recv`] function receives data from a connected socket or a bound connectionless
    /// socket.
    ///
    /// # Parameters
    ///  * `s` - The descriptor that identifies a connected socket.
    ///  * `buf` - A pointer to the buffer to receive the incoming data.
    ///  * `len` - The length, in bytes, of the buffer pointed to by the `buf` parameter.
    ///  * `flags` - A set of flags that influences the behavior of this function. See remarks
    ///              below. See the Remarks section for details on the possible value for this
    ///              parameter.
    ///
    /// # Return Value
    /// If no error occurs, [`recv`] returns the number of bytes received and the buffer pointed to
    /// by the `buf` parameter will contain this data received. If the connection has been
    /// gracefully closed, the return value is zero.
    ///
    /// Otherwise, a value of [`SOCKET_ERROR`] is returned, and a specific error code can be
    /// retrieved by calling [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`recv`] function is used to read incoming data on connection-oriented sockets, or
    /// connectionless sockets. When using a connection-oriented protocol, the sockets must be
    /// connected before calling [`recv`]. When using a connectionless protocol, the sockets must
    /// be bound before calling [`recv`].
    ///
    /// The local address of the socket must be known. For server applications, use an explicit
    /// [`bind`] function or an implicit [`accept`] or [`WSAAccept`] function. Explicit binding is
    /// discouraged for client applications. For client applications, the socket can become bound
    /// implicitly to a local address using [`connect`], [`WSAConnect`], [`sendto`], [`WSASendTo`],
    /// or [`WSAJoinLeaf`].
    ///
    /// For connected or connectionless sockets, the [`recv`] function restricts the addresses from
    /// which received messages are accepted. The function only returns messages from the remote
    /// address specified in the connection. Messages from other addresses are (silently)
    /// discarded.
    ///
    /// For connection-oriented sockets (type [`SOCK_STREAM`] for example), calling recv will
    /// return as much data as is currently available—up to the size of the buffer specified. If
    /// the socket has been configured for in-line reception of OOB data (socket option
    /// [`SO_OOBINLINE`]) and OOB data is yet unread, only OOB data will be returned. The
    /// application can use the [`ioctlsocket`] or [`WSAIoctlSIOCATMARK`] command to determine
    /// whether any more OOB data remains to be read.
    ///
    /// For connectionless sockets (type [`SOCK_DGRAM`] or other message-oriented sockets), data is
    /// extracted from the first enqueued datagram (message) from the destination address specified
    /// by the [`connect`] function.
    ///
    /// If the datagram or message is larger than the buffer specified, the buffer is filled with
    /// the first part of the datagram, and [`recv`] generates the error [`WSAEMSGSIZE`]. For
    /// unreliable protocols (for example, UDP) the excess data is lost; for reliable protocols,
    /// the data is retained by the service provider until it is successfully read by calling
    /// [`recv`] with a large enough buffer.
    ///
    /// If no incoming data is available at the socket, the [`recv`] call blocks and waits for data
    /// to arrive according to the blocking rules defined for [`WSARecv`] with the [`MSG_PARTIAL`]
    /// flag not set unless the socket is nonblocking. In this case, a value of [`SOCKET_ERROR`] is
    /// returned with the error code set to [`WSAEWOULDBLOCK`]. The select, [`WSAAsyncSelect`], or
    /// [`WSAEventSelect`] functions can be used to determine when more data arrives.
    ///
    /// If the socket is connection oriented and the remote side has shut down the connection
    /// gracefully, and all data has been received, a [`recv`] will complete immediately with zero
    /// bytes received. If the connection has been reset, a [`recv`] will fail with the error
    /// [`WSAECONNRESET`].
    ///
    /// The `flags` parameter can be used to influence the behavior of the function invocation
    /// beyond the options specified for the associated socket. The semantics of this function are
    /// determined by the socket options and the `flags` parameter. The possible value of `flags`
    /// parameter is constructed by using the bitwise OR operator with any of the following values:
    ///  * [`MSG_PEEK`] - Peeks at the incoming data. The data is copied into the buffer, but is
    ///                   not removed from the input queue.
    ///  * [`MSG_OOB`] - Processes Out Of Band (OOB) data.
    ///  * [`MSG_WAITALL`] - The receive request will complete only when one of the following
    ///                      events occurs:
    ///    - The buffer supplied by the caller is completely full.
    ///    - The connection has been closed.
    ///    - The request has been canceled or an error occurred.
    pub fn recv(s: SOCKET, buf: *mut c_char, len: c_int, flags: c_int) -> c_int;
}
