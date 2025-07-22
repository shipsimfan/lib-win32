use crate::{
    winsock2::{
        types::{LPWSABUF, LPWSAOVERLAPPED, LPWSAOVERLAPPED_COMPLETION_ROUTINE},
        SOCKET,
    },
    DWORD, LPDWORD,
};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::{
    accept, connect, recv, WSAGetLastError, SOCKET_ERROR, SOCK_DGRAM, SOCK_STREAM, WSABUF,
    WSAOVERLAPPED, WSA_IO_PENDING,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Ws2_32")]
extern "system" {
    /// The [`WSARecv`] function receives data from a connected socket or a bound connectionless
    /// socket.
    ///
    /// # Paramters
    ///  * `s` - A descriptor identifying a connected socket.
    ///  * `buffers` - A pointer to an array of [`WSABUF`] structures. Each [`WSABUF`] structure
    ///                contains a pointer to a buffer and the length, in bytes, of the buffer.
    ///  * `buffer_count` - The number of [`WSABUF`] structures in the `buffers` array.
    ///  * `number_of_bytes_recvd` - A pointer to the number, in bytes, of data received by this
    ///                              call if the receive operation completes immediately. Use
    ///                              [`null_mut`] for this parameter if the `overlapped` parameter
    ///                              is not [`null_mut`] to avoid potentially erroneous results.
    ///                              This parameter can be [`null_mut`] only if the `overlapped`
    ///                              parameter is not [`null_mut`].
    ///  * `flags` - A pointer to flags used to modify the behavior of the [`WSARecv`] function
    ///              call. For more information, see the Remarks section.
    ///  * `overlapped` - A pointer to a [`WSAOVERLAPPED`] structure (ignored for nonoverlapped
    ///                   sockets).
    ///  * `completion_routine` - A pointer to the completion routine called when the receive
    ///                           operation has been completed (ignored for nonoverlapped sockets).
    ///
    /// # Return Value
    /// If no error occurs and the receive operation has completed immediately, [`WSARecv`] returns
    /// zero. In this case, the completion routine will have already been scheduled to be called
    /// once the calling thread is in the alertable state. Otherwise, a value of [`SOCKET_ERROR`]
    /// is returned, and a specific error code can be retrieved by calling [`WSAGetLastError`]. The
    /// error code [`WSA_IO_PENDING`] indicates that the overlapped operation has been successfully
    /// initiated and that completion will be indicated at a later time. Any other error code
    /// indicates that the overlapped operation was not successfully initiated and no completion
    /// indication will occur.
    ///
    /// # Remarks
    /// The [`WSARecv`] function provides some additional features compared with the standard
    /// [`recv`] function in three important areas:
    ///  - It can be used in conjunction with overlapped sockets to perform overlapped [`recv`]
    ///    operations.
    ///  - It allows multiple receive buffers to be specified making it applicable to the
    ///    scatter/gather type of I/O.
    ///  - The `flags` parameter is used both on input and returned on output, allowing
    ///    applications to sense the output state of the [`MSG_PARTIAL`] flag bit. However, the
    ///    [`MSG_PARTIAL`] flag bit is not supported by all protocols.
    ///
    /// The [`WSARecv`] function is used on connected sockets or bound connectionless sockets
    /// specified by the `s` parameter and is used to read incoming data. The socket's local
    /// address must be known. For server applications, this is usually done explicitly through
    /// bind or implicitly through [`accept`] or [`WSAAccept`]. Explicit binding is discouraged for
    /// client applications. For client applications the socket can become bound implicitly to a
    /// local address through [`connect`], [`WSAConnect`], [`sendto`], [`WSASendTo`], or
    /// [`WSAJoinLeaf`].
    ///
    /// For connected, connectionless sockets, this function restricts the addresses from which
    /// received messages are accepted. The function only returns messages from the remote address
    /// specified in the connection. Messages from other addresses are (silently) discarded.
    ///
    /// For overlapped sockets, [`WSARecv`] is used to post one or more buffers into which incoming
    /// data will be placed as it becomes available, after which the application-specified
    /// completion indication (invocation of the completion routine or setting of an event object)
    /// occurs. If the operation does not complete immediately, the final completion status is
    /// retrieved through the completion routine or [`WSAGetOverlappedResult`].
    ///
    /// If both `overlapped` and `completion_routine` are [`null_mut`], the socket in this function
    /// will be treated as a nonoverlapped socket.
    ///
    /// For nonoverlapped sockets, the blocking semantics are identical to that of the standard
    /// [`recv`] function and the `overlapped` and `completion_routine` parameters are ignored. Any
    /// data that has already been received and buffered by the transport will be copied into the
    /// specified user buffers. In the case of a blocking socket with no data currently having been
    /// received and buffered by the transport, the call will block until data is received. Windows
    /// Sockets 2 does not define any standard blocking time-out mechanism for this function. For
    /// protocols acting as byte-stream protocols the stack tries to return as much data as
    /// possible subject to the available buffer space and amount of received data available.
    /// However, receipt of a single byte is sufficient to unblock the caller. There is no
    /// guarantee that more than a single byte will be returned. For protocols acting as
    /// message-oriented, a full message is required to unblock the caller.
    ///
    /// The buffers are filled in the order in which they appear in the array pointed to by
    /// `buffers`, and the buffers are packed so that no holes are created.
    ///
    /// If this function is completed in an overlapped manner, it is the Winsock service provider's
    /// responsibility to capture the [`WSABUF`] structures before returning from this call. This
    /// enables applications to build stack-based [`WSABUF`] arrays pointed to by the `buffers`
    /// parameter.
    ///
    /// For byte stream-style sockets (for example, type [`SOCK_STREAM`]), incoming data is placed
    /// into the buffers until the buffers are filled, the connection is closed, or the internally
    /// buffered data is exhausted. Regardless of whether or not the incoming data fills all the
    /// buffers, the completion indication occurs for overlapped sockets.
    ///
    /// For message-oriented sockets (for example, type [`SOCK_DGRAM`]), an incoming message is
    /// placed into the buffers up to the total size of the buffers, and the completion indication
    /// occurs for overlapped sockets. If the message is larger than the buffers, the buffers are
    /// filled with the first part of the message. If the [`MSG_PARTIAL`] feature is supported by
    /// the underlying service provider, the [`MSG_PARTIAL`] flag is set in `flags` and subsequent
    /// receive operations will retrieve the rest of the message. If [`MSG_PARTIAL`] is not
    /// supported but the protocol is reliable, [`WSARecv`] generates the error [`WSAEMSGSIZE`] and
    /// a subsequent receive operation with a larger buffer can be used to retrieve the entire
    /// message. Otherwise, (that is, the protocol is unreliable and does not support
    /// [`MSG_PARTIAL`]), the excess data is lost, and [`WSARecv`] generates the error
    /// [`WSAEMSGSIZE`].
    ///
    /// For connection-oriented sockets, [`WSARecv`] can indicate the graceful termination of the
    /// virtual circuit in one of two ways that depend on whether the socket is byte stream or
    /// message oriented. For byte streams, zero bytes having been read (as indicated by a zero
    /// return value to indicate success, and `number_of_bytes_recvd` value of zero) indicates
    /// graceful closure and that no more bytes will ever be read. For message-oriented sockets,
    /// where a zero byte message is often allowable, a failure with an error code of
    /// [`WSAEDISCON`] is used to indicate graceful closure. In any case a return error code of
    /// [`WSAECONNRESET`] indicates an abortive close has occurred.
    pub fn WSARecv(
        s: SOCKET,
        buffers: LPWSABUF,
        buffer_count: DWORD,
        number_of_bytes_recvd: LPDWORD,
        flags: LPDWORD,
        overlapped: LPWSAOVERLAPPED,
        completion_routine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
    ) -> c_int;
}
