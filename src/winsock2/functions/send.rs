use crate::winsock2::SOCKET;
use std::ffi::{c_char, c_int};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    winsock2::{
        WSAEventSelect, WSAGetLastError, AF_INET, AF_INET6, IPPROTO_UDP, SOCKET_ERROR, SOCK_DGRAM,
        SOCK_STREAM,
    },
    WSAEMSGSIZE,
};

#[link(name = "Ws2_32")]
extern "system" {
    /// The [`send`] function sends data on a connected socket.
    ///
    /// # Parameters
    ///  * `s` - A descriptor identifying a connected socket.
    ///  * `buf` - A pointer to a buffer containing the data to be transmitted.
    ///  * `len` - The length, in bytes, of the data in buffer pointed to by the `buf` parameter.
    ///  * `flags` - A set of flags that specify the way in which the call is made. This parameter
    ///              is constructed by using the bitwise OR operator with any of the following
    ///              values:
    ///    * [`MSG_DONTROUTE`] - Specifies that the data should not be subject to routing. A
    ///                          Windows Sockets service provider can choose to ignore this flag.
    ///    * [`MSG_OOB`] - Sends OOB data (stream-style socket such as [`SOCK_STREAM`] only).
    ///
    /// # Return Value
    /// If no error occurs, send returns the total number of bytes sent, which can be less than the
    /// number requested to be sent in the len parameter. Otherwise, a value of [`SOCKET_ERROR`] is
    /// returned, and a specific error code can be retrieved by calling [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`send`] function is used to write outgoing data on a connected socket.
    ///
    /// For message-oriented sockets (address family of [`AF_INET`] or [`AF_INET6`], type of
    /// [`SOCK_DGRAM`], and protocol of [`IPPROTO_UDP`], for example), care must be taken not to
    /// exceed the maximum packet size of the underlying provider. The maximum message packet size
    /// for a provider can be obtained by calling [`getsockopt`] with the optname parameter set to
    /// [`SO_MAX_MSG_SIZE`] to retrieve the value of socket option. If the data is too long to pass
    /// atomically through the underlying protocol, the error [`WSAEMSGSIZE`] is returned, and no
    /// data is transmitted.
    ///
    /// The successful completion of a [`send`] function does not indicate that the data was
    /// successfully delivered and received to the recipient. This function only indicates the data
    /// was successfully sent.
    ///
    /// If no buffer space is available within the transport system to hold the data to be
    /// transmitted, send will block unless the socket has been placed in nonblocking mode. On
    /// nonblocking stream oriented sockets, the number of bytes written can be between 1 and the
    /// requested length, depending on buffer availability on both the client and server computers.
    /// The [`select`], [`WSAAsyncSelect`] or [`WSAEventSelect`] functions can be used to determine
    /// when it is possible to send more data.
    ///
    /// Calling [`send`] with a `len` parameter of zero is permissible and will be treated by
    /// implementations as successful. In such cases, send will return zero as a valid value. For
    /// message-oriented sockets, a zero-length transport datagram is sent.
    ///
    /// The `flags` parameter can be used to influence the behavior of the function beyond the
    /// options specified for the associated socket. The semantics of the [`send`] function are
    /// determined by any options previously set on the socket specified in the `s` parameter and
    /// the `flags` parameter passed to the [`send`] function.
    ///
    /// The order of calls made to [`send`] is also the order in which the buffers are transmitted
    /// to the transport layer. send should not be called on the same stream-oriented socket
    /// concurrently from different threads, because some Winsock providers may split a large send
    /// request into multiple transmissions, and this may lead to unintended data interleaving from
    /// multiple concurrent send requests on the same stream-oriented socket.
    pub fn send(s: SOCKET, buf: *const c_char, len: c_int, flags: c_int) -> c_int;
}
