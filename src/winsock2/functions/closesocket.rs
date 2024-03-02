use crate::winsock2::SOCKET;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{winsock2::WSAGetLastError, WSAECONNRESET, WSAENOTSOCK, WSAEWOULDBLOCK};

#[link(name = "Ws2_32")]
extern "system" {
    /// The [`closesocket`] function closes an existing socket.
    ///
    /// # Parameters
    ///  * `s` - A descriptor identifying the socket to close.
    ///
    /// # Return Value
    /// If no error occurs, [`closesocket`] returns zero. Otherwise, a value of [`SOCKET_ERROR`] is
    /// returned, and a specific error code can be retrieved by calling [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`closesocket`] function closes a socket. Use it to release the socket descriptor
    /// passed in the `s` parameter. Note that the socket descriptor passed in the `s` parameter
    /// may immediately be reused by the system as soon as [`closesocket`] function is issued. As a
    /// result, it is not reliable to expect further references to the socket descriptor passed in
    /// the `s` parameter to fail with the error [`WSAENOTSOCK`]. A Winsock client must never issue
    /// [`closesocket`] on `s` concurrently with another Winsock function call.
    ///
    /// Any pending overlapped send and receive operations ([`WSASend`]/[`WSASendTo`]/[`WSARecv`]/
    /// [`WSARecvFrom`] with an overlapped socket) issued by any thread in this process are also
    /// canceled. Any event, completion routine, or completion port action specified for these
    /// overlapped operations is performed. The pending overlapped operations fail with the error
    /// status [`WSA_OPERATION_ABORTED`].
    ///
    /// An application should not assume that any outstanding I/O operations on a socket will all
    /// be guaranteed to completed when closesocket returns. The closesocket function will initiate
    /// cancellation on the outstanding I/O operations, but that does not mean that an application
    /// will receive I/O completion for these I/O operations by the time the closesocket function
    /// returns. Thus, an application should not cleanup any resources ([`WSAOVERLAPPED`]
    /// structures, for example) referenced by the outstanding I/O requests until the I/O requests
    /// are indeed completed.
    ///
    /// An application should always have a matching call to [`closesocket`] for each successful
    /// call to socket to return any socket resources to the system.
    ///
    /// The [`LINGER`] structure maintains information about a specific socket that specifies how
    /// that socket should behave when data is queued to be sent and the [`closesocket`] function
    /// is called on the socket.
    ///
    /// The `l_onoff` member of the [`LINGER`] structure determines whether a socket should remain
    /// open for a specified amount of time after a [`closesocket`] function call to enable queued
    /// data to be sent. This member can be modified in two ways:
    ///  - Call the [`setsockopt`] function with the `optname` parameter set to [`SO_DONTLINGER`].
    ///    The `optval` parameter determines how the `l_onoff` member is modified.
    ///  - Call the [`setsockopt`] function with the `optname` parameter set to [`SO_LINGER`]. The
    ///    `optval` parameter specifies how both the `l_onoff` and `l_linger` members are modified.
    ///
    /// The `l_linger` member of the [`LINGER`] structure determines the amount of time, in
    /// seconds, a socket should remain open. This member is only applicable if the `l_onoff`
    /// member of the [`LINGER`] structure is nonzero.
    ///
    /// The default parameters for a socket are the `l_onoff` member of the [`LINGER`] structure is
    /// zero, indicating that the socket should not remain open. The default value for the
    /// `l_linger` member of the [`LINGER`] structure is zero, but this value is ignored when the
    /// `l_onoff` member is set to zero.
    ///
    /// To enable a socket to remain open, an application should set the `l_onoff` member to a
    /// nonzero value and set the `l_linger` member to the desired timeout in seconds. To disable a
    /// socket from remaining open, an application only needs to set the `l_onoff` member of the
    /// [`LINGER`] structure to zero.
    ///
    /// If an application calls the [`setsockopt`] function with the `optname` parameter set to
    /// [`SO_DONTLINGER`] to set the `l_onoff` member to a nonzero value, the value for the
    /// `l_linger` member is not specified. In this case, the timeout used is implementation
    /// dependent. If a previous timeout has been established for a socket (by previously calling
    /// the [`setsockopt`] function with the optname parameter set to [`SO_LINGER`]), this timeout
    /// value should be reinstated by the service provider.
    ///
    /// The semantics of the [`closesocket`] function are affected by the socket options that set
    /// members of [`LINGER`] structure:
    ///  * `l_onoff == 0` - Graceful close. Does not wait.
    ///  * `l_onoff != 0 && l_linger == 0` - Hard close. Does not wait.
    ///  * `l_onoff != 0 && l_linger != 0` - Graceful if all data is sent within timeout value
    ///                                      specified in the `l_linger` member. Hard if all data
    ///                                      could not be sent within timeout value specified in
    ///                                      the `l_linger` member. Waits for close.
    ///
    /// If the `l_onoff` member of the [`LINGER`] structure is zero on a stream socket, the
    /// [`closesocket`] call will return immediately and does not receive [`WSAEWOULDBLOCK`]
    /// whether the socket is blocking or nonblocking. However, any data queued for transmission
    /// will be sent, if possible, before the underlying socket is closed. This is also called a
    /// graceful disconnect or close. In this case, the Windows Sockets provider cannot release the
    /// socket and other resources for an arbitrary period, thus affecting applications that expect
    /// to use all available sockets. This is the default behavior for a socket.
    ///
    /// If the `l_onoff` member of the [`LINGER`] structure is nonzero and `l_linger` member is
    /// zero, [`closesocket`] is not blocked even if queued data has not yet been sent or
    /// acknowledged. This is called a hard or abortive close, because the socket's virtual circuit
    /// is reset immediately, and any unsent data is lost. On Windows, any [`recv`] call on the
    /// remote side of the circuit will fail with [`WSAECONNRESET`].
    ///
    /// If the `l_onoff` member of the [`LINGER`] structure is set to nonzero and `l_linger` member
    /// is set to a nonzero timeout on a blocking socket, the [`closesocket`] call blocks until the
    /// remaining data has been sent or until the timeout expires. This is called a graceful
    /// disconnect or close if all of the data is sent within timeout value specified in the
    /// `l_linger` member. If the timeout expires before all data has been sent, the Windows
    /// Sockets implementation terminates the connection before [`closesocket`] returns and this is
    /// called a hard or abortive close.
    ///
    /// Setting the `l_onoff` member of the [`LINGER`] structure to nonzero and the `l_linger`
    /// member with a nonzero timeout interval on a nonblocking socket is not recommended. In this
    /// case, the call to [`closesocket`] will fail with an error of [`WSAEWOULDBLOCK`] if the
    /// close operation cannot be completed immediately. If [`closesocket`] fails with
    /// [`WSAEWOULDBLOCK`] the socket handle is still valid, and a disconnect is not initiated. The
    /// application must call [`closesocket`] again to close the socket.
    ///
    /// If the `l_onoff` member of the [`LINGER`] structure is nonzero and the `l_linger` member is
    /// a nonzero timeout interval on a blocking socket, the result of the [`closesocket`] function
    /// can't be used to determine whether all data has been sent to the peer. If the data is sent
    /// before the timeout specified in the `l_linger` member expires or if the connection was
    /// aborted, the [`closesocket`] function won't return an error code (the return value from the
    /// [`closesocket`] function is zero).
    ///
    /// The [`closesocket`] call will only block until all data has been delivered to the peer or
    /// the timeout expires. If the connection is reset because the timeout expires, then the
    /// socket will not go into `TIME_WAIT` state. If all data is sent within the timeout period,
    /// then the socket can go into `TIME_WAIT` state.
    ///
    /// If the `l_onoff` member of the [`LINGER`] structure is nonzero and the `l_linger` member is
    /// a zero timeout interval on a blocking socket, then a call to [`closesocket`] will reset the
    /// connection. The socket will not go to the `TIME_WAIT` state.
    ///
    /// The [`getsockopt`] function can be called with the `optname` parameter set to [`SO_LINGER`]
    /// to retrieve the current value of the linger structure associated with a socket.
    pub fn closesocket(s: SOCKET) -> c_int;
}
