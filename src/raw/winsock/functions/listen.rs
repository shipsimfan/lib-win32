use crate::raw::Socket;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{bind, WSAGetLastError, SOCKET_ERROR, SOCK_STREAM};

#[link(name = "Ws2_32")]
extern "C" {
    /// # listen function (winsock2.h)
    ///
    /// The [`listen`] function places a socket in a state in which it is
    /// listening for an incoming connection.
    ///
    /// ## Parameters
    /// `s`\
    /// A descriptor identifying a bound, unconnected socket.
    ///
    /// `back_log`\
    /// The maximum length of the queue of pending connections. If set to
    /// [`SOMAXCONN`], the underlying service provider responsible for socket
    /// `s` will set the backlog to a maximum reasonable value. If set to
    /// [`SOMAXCONN_HINT(N)`] (where `N` is a number), the backlog value will
    /// be `N`, adjusted to be within the range (200, 65535). Note that
    /// [`SOMAXCONN_HINT`] can be used to set the backlog to a larger value
    /// than possible with [`SOMAXCONN`].
    ///
    /// [`SOMAXCONN_HINT`] is only supported by the Microsoft TCP/IP service
    /// provider. There is no standard provision to obtain the actual backlog
    /// value.
    ///
    /// ## Return Value
    /// If no error occurs, [`listen`] returns zero. Otherwise, a value of
    /// [`SOCKET_ERROR`] is returned, and a specific error code can be
    /// retrieved by calling [`WSAGetLastError`].
    ///
    /// ## Remarks
    /// To accept connections, a socket is first created with the socket
    /// function and bound to a local address with the [`bind`] function. A
    /// backlog for incoming connections is specified with [`listen`], and then
    /// the connections are accepted with the [`accept`] function. Sockets that
    /// are connection oriented, those of type [`SOCK_STREAM`] for example, are
    /// used with [`listen`]. The socket `s` is put into passive mode where
    /// incoming connection requests are acknowledged and queued pending
    /// acceptance by the process.
    ///
    /// A value for the backlog of [`SOMAXCONN`] is a special constant that
    /// instructs the underlying service provider responsible for socket `s` to
    /// set the length of the queue of pending connections to a maximum
    /// reasonable value.
    ///
    /// On Windows Sockets 2, this maximum value defaults to a large value
    /// (typically several hundred or more).
    ///
    /// When calling the [`listen`] function in a Bluetooth application, it is
    /// strongly recommended that a much lower value be used for the backlog
    /// parameter (typically 2 to 4), since only a few client connections are
    /// accepted. This reduces the system resources that are allocated for use
    /// by the listening socket. This same recommendation applies to other
    /// network applications that expect only a few client connections.
    ///
    /// The [`listen`] function is typically used by servers that can have more
    /// than one connection request at a time. If a connection request arrives
    /// and the queue is full, the client will receive an error with an
    /// indication of [`WSAECONNREFUSED`].
    ///
    /// If there are no available socket descriptors, listen attempts to
    /// continue to function. If descriptors become available, a later call to
    /// [`listen`] or [`accept`] will refill the queue to the current or most
    /// recent value specified for the backlog parameter, if possible, and
    /// resume listening for incoming connections.
    ///
    /// If the [`listen`] function is called on an already listening socket, it
    /// will return success without changing the value for the backlog
    /// parameter. Setting the backlog parameter to 0 in a subsequent call to
    /// [`listen`] on a listening socket is not considered a proper reset,
    /// especially if there are connections on the socket.
    pub fn listen(s: Socket, back_log: c_int) -> c_int;
}

pub const SOMAXCONN: c_int = 5;
