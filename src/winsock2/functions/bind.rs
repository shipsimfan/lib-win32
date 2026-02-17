use crate::winsock2::{sockaddr, SOCKET};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    winsock2::{sockaddr_in, sockaddr_in6, socket, WSAGetLastError, SOCKET_ERROR, SOCK_RAW},
    WSAEFAULT,
};
use std::ffi::c_int;

#[link(name = "Ws2_32")]
unsafe extern "system" {
    /// The [`bind`] function associates a local address with a socket
    ///
    /// # Parameters
    ///  * `s` - A descriptor identifying an unbound socket.
    ///  * `addr` - A pointer to a [`sockaddr`] structure of the local address to assign to the
    ///             bound socket.
    ///  * `namelen` - The length, in bytes, of the value pointed to by `addr`.
    ///
    /// # Return Value
    /// If no error occurs, [`bind`] returns zero. Otherwise, it returns [`SOCKET_ERROR`], and a
    /// specific error code can be retrieved by calling [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`bind`] function is required on an unconnected socket before subsequent calls to the
    /// [`listen`] function. It is normally used to bind to either connection-oriented (stream) or
    /// connectionless (datagram) sockets. The [`bind`] function may also be used to bind to a raw
    /// socket (the socket was created by calling the socket function with the type parameter set
    /// to [`SOCK_RAW`]). The [`bind`] function may also be used on an unconnected socket before
    /// subsequent calls to the [`connect`], [`ConnectEx`], [`WSAConnect`], [`WSAConnectByList`],
    /// or [`WSAConnectByName`] functions before send operations.
    ///
    /// When a socket is created with a call to the [`socket`] function, it exists in a namespace
    /// (address family), but it has no name assigned to it. Use the [`bind`] function to establish
    /// the local association of the socket by assigning a local name to an unnamed socket.
    ///
    /// A name consists of three parts when using the Internet address family:
    ///  - The address family.
    ///  - A host address.
    ///  - A port number that identifies the application.
    ///
    /// In Windows Sockets 2, the `addr` parameter is not strictly interpreted as a pointer to a
    /// [`sockaddr`] structure. It is cast this way for Windows Sockets 1.1 compatibility. Service
    /// providers are free to regard it as a pointer to a block of memory of size `namelen`. The
    /// first 2 bytes in this block (corresponding to the `family` member of the [`sockaddr`],
    /// [`sockaddr_in`], or [`sockaddr_in6`] structures) must contain the address family that was
    /// used to create the socket. Otherwise, an error [`WSAEFAULT`] occurs.
    ///
    /// If an application does not care what local address is assigned, specify the constant value
    /// [`INADDR_ANY`] for an IPv4 local address or the constant value [`in6addr_any`] for an IPv6
    /// local address in the `data` member of the `addr` parameter. This allows the underlying
    /// service provider to use any appropriate network address, potentially simplifying
    /// application programming in the presence of multihomed hosts (that is, hosts that have more
    /// than one network interface and address).
    ///
    /// For TCP/IP, if the port is specified as zero, the service provider assigns a unique port to
    /// the application from the dynamic client port range. On Windows Vista and later, the dynamic
    /// client port range is a value between 49152 and 65535. This is a change from Windows Server
    /// 2003 and earlier where the dynamic client port range was a value between 1025 and 5000. The
    /// maximum value for the client dynamic port range can be changed by setting a value under the
    /// following registry key: `HKLM\SYSTEM\CurrentControlSet\Services\Tcpip\Parameters`
    ///
    /// The `MaxUserPort` registry value sets the value to use for the maximum value of the dynamic
    /// client port range. You must restart the computer for this setting to take effect.
    ///
    /// On Windows Vista and later, the dynamic client port range can be viewed and changed using
    /// `netsh` commands. The dynamic client port range can be set differently for UDP and TCP and
    /// also for IPv4 and IPv6.
    ///
    /// The application can use [`getsockname`] after calling bind to learn the address and the
    /// port that has been assigned to the socket. If the Internet address is equal to
    /// [`INADDR_ANY`] or [`in6addr_any`], [`getsockname`] cannot necessarily supply the address
    /// until the socket is connected, since several addresses can be valid if the host is
    /// multihomed. Binding to a specific port number other than port 0 is discouraged for client
    /// applications, since there is a danger of conflicting with another socket already using that
    /// port number on the local computer.
    ///
    /// For multicast operations, the preferred method is to call the bind function to associate a
    /// socket with a local IP address and then join the multicast group. Although this order of
    /// operations is not mandatory, it is strongly recommended. So a multicast application would
    /// first select an IPv4 or IPv6 address on the local computer, the wildcard IPv4 address
    /// ([`INADDR_ANY`]), or the wildcard IPv6 address ([`in6addr_any`]). The multicast application
    /// would then call the bind function with this address in the in the `data` member of the
    /// `addr` parameter to associate the local IP address with the socket. If a wildcard address
    /// was specified, then Windows will select the local IP address to use. After the [`bind`]
    /// function completes, an application would then join the multicast group of interest. This
    /// socket can then be used to receive multicast packets from the multicast group using the
    /// [`recv`], [`recvfrom`], [`WSARecv`], [`WSARecvEx`], [`WSARecvFrom`], or [`LPFN_WSARECVMSG`]
    /// ([`WSARecvMsg`]) functions.
    ///
    /// The [`bind`] function is not normally required for send operations to a multicast group.
    /// The [`sendto`], [`WSASendMsg`], and [`WSASendTo`] functions implicitly bind the socket to
    /// the wildcard address if the socket is not already bound. The [`bind`] function is required
    /// before the use of the [`send`] or [`WSASend`] functions which do not perform an implicit
    /// bind and are allowed only on connected sockets, which means the socket must have already
    /// been bound for it to be connected. The bind function might be used before send operations
    /// using the [`sendto`], [`WSASendMsg`], or [`WSASendTo`] functions if an application wanted
    /// to select a specific local IP address on a local computer with multiple network interfaces
    /// and local IP addresses. Otherwise an implicit bind to the wildcard address using the
    /// [`sendto`], [`WSASendMsg`], or [`WSASendTo`] functions might result in a different local IP
    /// address being used for send operations.
    pub fn bind(s: SOCKET, addr: *const sockaddr, namelen: c_int) -> c_int;
}
