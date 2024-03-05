use crate::{
    winsock2::{GROUP, LPWSAPROTOCOL_INFOW, SOCKET},
    DWORD,
};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::{
    bind, WSAGetLastError, WSASocket, AF_APPLETALK, AF_BTH, AF_INET, AF_INET6, AF_IPX, AF_IRDA,
    AF_NETBIOS, AF_UNSPEC, BTHPROTO_RFCOMM, INVALID_SOCKET, IPPROTO_ICMP, IPPROTO_ICMPV6,
    IPPROTO_IGMP, IPPROTO_RM, IPPROTO_TCP, IPPROTO_UDP, SG_CONSTRAINED_GROUP,
    SG_UNCONSTRAINED_GROUP, SOCK_DGRAM, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET, SOCK_STREAM,
    WSA_FLAG_ACCESS_SYSTEM_SECURITY, WSA_FLAG_MULTIPOINT_C_LEAF, WSA_FLAG_MULTIPOINT_C_ROOT,
    WSA_FLAG_MULTIPOINT_D_LEAF, WSA_FLAG_NO_HANDLE_INHERIT, WSA_FLAG_OVERLAPPED,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Ws2_32")]
extern "system" {
    /// The [`WSASocket`] function creates a socket that is bound to a specific transport-service provider.
    ///
    ///
    /// # Parameters
    ///  * `af` - The address family specification. The values currently supported are [`AF_INET`]
    ///           or [`AF_INET6`], which are the Internet address family formats for IPv4 and IPv6.
    ///           Other options for address family ([`AF_NETBIOS`] for use with NetBIOS, for
    ///           example) are supported if a Windows Sockets service provider for the address
    ///           family is installed. Note that the values for the `AF_` address family and `PF_`
    ///           protocol family constants are identical (for example, [`AF_INET`] and
    ///           [`PF_INET`]), so either constant can be used. Below are the common values
    ///           for address family although many other values are possible:
    ///    * [`AF_UNSPEC`] - The address family is unspecified.
    ///    * [`AF_INET`] - The Internet Protocol version 4 (IPv4) address family.
    ///    * [`AF_IPX`] - The IPX/SPX address family. This address family is only supported if the
    ///                   NWLink IPX/SPX NetBIOS Compatible Transport protocol is installed. This
    ///                   address family is not supported on Windows Vista and later.
    ///    * [`AF_APPLETALK`] - The AppleTalk address family. This address family is only supported
    ///                         if the AppleTalk protocol is installed. This address family is not
    ///                         supported on Windows Vista and later.
    ///    * [`AF_NETBIOS`] - The NetBIOS address family. This address family is only supported if
    ///                       the Windows Sockets provider for NetBIOS is installed. The Windows
    ///                       Sockets provider for NetBIOS is supported on 32-bit versions of
    ///                       Windows. This provider is installed by default on 32-bit versions of
    ///                       Windows. The Windows Sockets provider for NetBIOS is not supported on
    ///                       64-bit versions of windows including Windows 7, Windows Server 2008,
    ///                       Windows Vista, Windows Server 2003, or Windows XP. The Windows
    ///                       Sockets provider for NetBIOS only supports sockets where the type
    ///                       parameter is set to [`SOCK_DGRAM`]. The Windows Sockets provider for
    ///                       NetBIOS is not directly related to the NetBIOS programming interface.
    ///                       The NetBIOS programming interface is not supported on Windows Vista,
    ///                       Windows Server 2008, and later.
    ///    * [`AF_INET6`] - The Internet Protocol version 6 (IPv6) address family.
    ///    * [`AF_IRDA`] -	The Infrared Data Association (IrDA) address family. This address
    ///                     family is only supported if the computer has an infrared port and
    ///                     driver installed.
    ///    * [`AF_BTH`] - The Bluetooth address family. This address family is supported on Windows
    ///                   XP with SP2 or later if the computer has a Bluetooth adapter and driver
    ///                   installed.
    ///  * `r#type` - The type specification for the new socket. The following table lists the
    ///               possible values for the type parameter supported for Windows Sockets 2:
    ///    * [`SOCK_STREAM`] - A socket type that provides sequenced, reliable, two-way,
    ///                        connection-based byte streams with an OOB data transmission
    ///                        mechanism. This socket type uses the Transmission Control Protocol
    ///                        (TCP) for the Internet address family ([`AF_INET`] or [`AF_INET6`]).
    ///    * [`SOCK_DGRAM`] - A socket type that supports datagrams, which are connectionless,
    ///                       unreliable buffers of a fixed (typically small) maximum length. This
    ///                       socket type uses the User Datagram Protocol (UDP) for the Internet
    ///                       address family ([`AF_INET`] or [`AF_INET6`]).
    ///    * [`SOCK_RAW`] - A socket type that provides a raw socket that allows an application to
    ///                     manipulate the next upper-layer protocol header. To manipulate the IPv4
    ///                     header, the [`IP_HDRINCL`] socket option must be set on the socket. To
    ///                     manipulate the IPv6 header, the [`IPV6_HDRINCL`] socket option must be
    ///                     set on the socket.
    ///    * [`SOCK_RDM`] - A socket type that provides a reliable message datagram. An example of
    ///                     this type is the Pragmatic General Multicast (PGM) multicast protocol
    ///                     implementation in Windows, often referred to as reliable multicast
    ///                     programming. This type value is only supported if the Reliable
    ///                     Multicast Protocol is installed.
    ///    * [`SOCK_SEQPACKET`] - A socket type that provides a pseudo-stream packet based on
    ///                           datagrams.
    ///  * `protocol` - The protocol to be used. The possible options for the protocol parameter
    ///                 are specific to the address family and socket type specified. If a value of
    ///                 0 is specified, the caller does not wish to specify a protocol and the
    ///                 service provider will choose the protocol to use. When the af parameter is
    ///                 [`AF_INET`] or [`AF_INET6`] and the type is [`SOCK_RAW`], the value
    ///                 specified for the protocol is set in the protocol field of the IPv6 or IPv4
    ///                 packet header. Below are the common values for the protocol although many
    ///                 other values are possible:
    ///    * [`IPPROTO_ICMP`] - The Internet Control Message Protocol (ICMP). This is a possible
    ///                         value when the `af` parameter is [`AF_UNSPEC`], [`AF_INET`], or
    ///                         [`AF_INET6`] and the `type` parameter is [`SOCK_RAW`] or
    ///                         unspecified. This protocol value is supported on Windows XP and
    ///                         later.
    ///    * [`IPPROTO_IGMP`] - The Internet Group Management Protocol (IGMP). This is a possible
    ///                         value when the `af` parameter is [`AF_UNSPEC`], [`AF_INET`], or
    ///                         [`AF_INET6`] and the `type` parameter is [`SOCK_RAW`] or
    ///                         unspecified. This protocol value is supported on Windows XP and
    ///                         later.
    ///    * [`BTHPROTO_RFCOMM`] - The Bluetooth Radio Frequency Communications (Bluetooth RFCOMM)
    ///                            protocol. This is a possible value when the af parameter is
    ///                            [`AF_BTH`] and the `type` parameter is [`SOCK_STREAM`]. This
    ///                            protocol value is supported on Windows XP with SP2 or later.
    ///    * [`IPPROTO_TCP`] - The Transmission Control Protocol (TCP). This is a possible value
    ///                        when the `af` parameter is [`AF_INET`] or [`AF_INET6`] and the
    ///                        `type` parameter is [`SOCK_STREAM`].
    ///    * [`IPPROTO_UDP`] - The User Datagram Protocol (UDP). This is a possible value when the
    ///                        `af` parameter is [`AF_INET`] or [`AF_INET6`] and the `type`
    ///                        parameter is [`SOCK_DGRAM`].
    ///    * [`IPPROTO_ICMPV6`] - The Internet Control Message Protocol Version 6 (ICMPv6). This is
    ///                           a possible value when the `af` parameter is [`AF_UNSPEC`],
    ///                           [`AF_INET`], or [`AF_INET6`] and the `type` parameter is
    ///                           [`SOCK_RAW`] or unspecified. This protocol value is supported on
    ///                           Windows XP and later.
    ///    * [`IPPROTO_RM`] - The PGM protocol for reliable multicast. This is a possible value
    ///                       when the `af` parameter is [`AF_INET`] and the `type` parameter is
    ///                       [`SOCK_RDM`]. On the Windows SDK released for Windows Vista and
    ///                       later, this protocol is also called [`IPPROTO_PGM`]. This protocol
    ///                       value is only supported if the Reliable Multicast Protocol is
    ///                       installed.
    ///  * `protocol_info` - A pointer to a [`WSAPROTOCOL_INFO`] structure that defines the
    ///                      characteristics of the socket to be created. If this parameter is not
    ///                      [`null_mut`], the socket will be bound to the provider associated with
    ///                      the indicated [`WSAPROTOCOL_INFO`] structure.
    ///  * `g` - An existing socket group ID or an appropriate action to take when creating a new
    ///          socket and a new socket group. If g is an existing socket group ID, join the new
    ///          socket to this socket group, provided all the requirements set by this group are
    ///          met. If `g` is not an existing socket group ID, then the following values are
    ///          possible:
    ///    * 0 - No group operation is performed.
    ///    * [`SG_UNCONSTRAINED_GROUP`] - Create an unconstrained socket group and have the new
    ///                                   socket be the first member. For an unconstrained group,
    ///                                   Winsock does not constrain all sockets in the socket
    ///                                   group to have been created with the same value for the
    ///                                   type and protocol parameters.
    ///    * [`SG_CONSTRAINED_GROUP`] - Create a constrained socket group and have the new socket
    ///                                 be the first member. For a constrained socket group,
    ///                                 Winsock constrains all sockets in the socket group to have
    ///                                 been created with the same value for the type and protocol
    ///                                 parameters. A constrained socket group may consist only of
    ///                                 connection-oriented sockets, and requires that connections
    ///                                 on all grouped sockets be to the same address on the same
    ///                                 host.
    ///  * `flags` - A set of flags used to specify additional socket attributes. A combination of
    ///              these flags may be set, although some combinations are not allowed.
    ///    * [`WSA_FLAG_OVERLAPPED`] - Create a socket that supports overlapped I/O operations.
    ///                                Most sockets should be created with this flag set.
    ///                                Overlapped sockets can utilize [`WSASend`], [`WSASendTo`],
    ///                                [`WSARecv`], [`WSARecvFrom`], and [`WSAIoctl`] for
    ///                                overlapped I/O operations, which allow multiple operations
    ///                                to be initiated and in progress simultaneously. All
    ///                                functions that allow overlapped operation ([`WSASend`],
    ///                                [`WSARecv`], [`WSASendTo`], [`WSARecvFrom`], [`WSAIoctl`])
    ///                                also support nonoverlapped usage on an overlapped socket if
    ///                                the values for parameters related to overlapped operations
    ///                                are [`null_mut`].
    ///    * [`WSA_FLAG_MULTIPOINT_C_ROOT`] - Create a socket that will be a c_root in a multipoint
    ///                                       session. This attribute is only allowed if the
    ///                                       [`WSAPROTOCOL_INFO`] structure for the transport
    ///                                       provider that creates the socket supports a
    ///                                       multipoint or multicast mechanism and the control
    ///                                       plane for a multipoint session is rooted. This would
    ///                                       be indicated by the `service_flags1` member of the
    ///                                       [`WSAPROTOCOL_INFO`] structure with the
    ///                                       [`XP1_SUPPORT_MULTIPOINT`] and
    ///                                       [`XP1_MULTIPOINT_CONTROL_PLANE`] flags set. When the
    ///                                       `protocol_info` parameter is not [`null_mut`], the
    ///                                       [`WSAPROTOCOL_INFO`] structure for the transport
    ///                                       provider is pointed to by the `protocol_info`
    ///                                       parameter. When the `protocol_info` parameter is
    ///                                       [`null_mut`], the [`WSAPROTOCOL_INFO`] structure is
    ///                                       based on the transport provider selected by the
    ///                                       values specified for the `af`, `type`, and `protocol`
    ///                                       parameters.
    ///    * [`WSA_FLAG_MULTIPOINT_C_LEAF`] - Create a socket that will be a c_leaf in a multipoint
    ///                                       session. This attribute is only allowed if the
    ///                                       [`WSAPROTOCOL_INFO`] structure for the transport
    ///                                       provider that creates the socket supports a
    ///                                       multipoint or multicast mechanism and the control
    ///                                       plane for a multipoint session is non-rooted. This
    ///                                       would be indicated by the `service_flags1` member of
    ///                                       the [`WSAPROTOCOL_INFO`] structure with the
    ///                                       [`XP1_SUPPORT_MULTIPOINT`] flag set and the
    ///                                       [`XP1_MULTIPOINT_CONTROL_PLANE`] flag not set. When
    ///                                       the `protocol_info` parameter is not [`null_mut`],
    ///                                       the [`WSAPROTOCOL_INFO`] structure for the transport
    ///                                       provider is pointed to by the `protocol_info`
    ///                                       parameter. When the `protocol_info` parameter is
    ///                                       [`null_mut], the [`WSAPROTOCOL_INFO`] structure is
    ///                                       based on the transport provider selected by the
    ///                                       values specified for the `af`, `type`, and `protocol`
    ///                                       parameters.
    ///    * [`WSA_FLAG_MULTIPOINT_D_LEAF`] - Create a socket that will be a d_leaf in a multipoint
    ///                                       session. This attribute is only allowed if the
    ///                                       [`WSAPROTOCOL_INFO`] structure for the transport
    ///                                       provider that creates the socket supports a
    ///                                       multipoint or multicast mechanism and the data plane
    ///                                       for a multipoint session is non-rooted. This would be
    ///                                       indicated by the `service_flags1` member of the
    ///                                       [`WSAPROTOCOL_INFO`] structure with the
    ///                                       [`XP1_SUPPORT_MULTIPOINT`] flag set and the
    ///                                       [`XP1_MULTIPOINT_DATA_PLANE`] flag not set. When the
    ///                                       `protocol_info` parameter is not [`null_mut`], the
    ///                                       [`WSAPROTOCOL_INFO`] structure for the transport
    ///                                       provider is pointed to by the `protocol_info`
    ///                                       parameter. When the `protocol_info` parameter is
    ///                                       [`null_mut`], the [`WSAPROTOCOL_INFO`] structure is
    ///                                       based on the transport provider selected by the
    ///                                       values specified for the `af`, `type`, and `protocol`
    ///                                       parameters.
    ///    * [`WSA_FLAG_ACCESS_SYSTEM_SECURITY`] - Create a socket that allows the ability to set a
    ///                                            security descriptor on the socket that contains
    ///                                            a security access control list (SACL) as opposed
    ///                                            to just a discretionary access control list
    ///                                            (DACL). SACLs are used for generating audits and
    ///                                            alarms when an access check occurs on the
    ///                                            object. For a socket, an access check occurs to
    ///                                            determine whether the socket should be allowed
    ///                                            to bind to a specific address specified to the
    ///                                            [`bind`] function. The `ACCESS_SYSTEM_SECURITY`
    ///                                            access right controls the ability to get or set
    ///                                            the SACL in an object's security descriptor. The
    ///                                            system grants this access right only if the
    ///                                            `SE_SECURITY_NAME` privilege is enabled in the
    ///                                            access token of the requesting thread.
    ///    * [`WSA_FLAG_NO_HANDLE_INHERIT`] - Create a socket that is non-inheritable. A socket
    ///                                       handle created by the [`WSASocket`] or the [`socket`]
    ///                                       function is inheritable by default. When this flag is
    ///                                       set, the socket handle is non-inheritable. The
    ///                                       [`GetHandleInformation`] function can be used to
    ///                                       determine if a socket handle was created with the
    ///                                       [`WSA_FLAG_NO_HANDLE_INHERIT`] flag set. The
    ///                                       [`GetHandleInformation`] function will return that
    ///                                       the [`HANDLE_FLAG_INHERIT`] value is set. This flag
    ///                                       is supported on Windows 7 with SP1, Windows Server
    ///                                       2008 R2 with SP1, and later
    ///
    /// # Return Value
    /// If no error occurs, [`WSASocket`] returns a descriptor referencing the new socket.
    /// Otherwise, a value of [`INVALID_SOCKET`] is returned, and a specific error code can be
    /// retrieved by calling [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`WSASocket`] function causes a socket descriptor and any related resources to be
    /// allocated and associated with a transport-service provider. Most sockets should be created
    /// with the [`WSA_FLAG_OVERLAPPED`] attribute set in the `flags` parameter. A socket created
    /// with this attribute supports the use of overlapped I/O operations which provide higher
    /// performance. By default, a socket created with the [`WSASocket`] function will not have
    /// this overlapped attribute set. In contrast, the [`socket`] function creates a socket that
    /// supports overlapped I/O operations as the default behavior.
    ///
    /// If the `protocol_info` parameter is [`null_mut`], Winsock will utilize the first available
    /// transport-service provider that supports the requested combination of address family,
    /// socket type and protocol specified in the `af`, `type`, and `protocol` parameters.
    ///
    /// If the `protocol_info` parameter is not [`null_mut`], the socket will be bound to the
    /// provider associated with the indicated [`WSAPROTOCOL_INFO`] structure. In this instance,
    /// the application can supply the manifest constant [`FROM_PROTOCOL_INFO`] as the value for
    /// any of `af`, `type`, or `protocol` parameters. This indicates that the corresponding values
    /// from the indicated [`WSAPROTOCOL_INFO`] structure (`address_family`, `socket_type`,
    /// `protocol`) are to be assumed. In any case, the values specified for `af`, `type`, and
    /// `protocol` are passed unmodified to the transport-service provider.
    ///
    /// When selecting a protocol and its supporting service provider based on `af`, `type`, and
    /// `protocol`, this procedure will only choose a base protocol or a protocol chain, not a
    /// protocol layer by itself. Unchained protocol layers are not considered to have partial
    /// matches on type or af, either. That is, they do not lead to an error code of
    /// [`WSAEAFNOSUPPORT`] or [`WSAEPROTONOSUPPORT`], if no suitable protocol is found.
    ///
    /// Applications are encouraged to use [`AF_INET6`] for the `af` parameter and create a
    /// dual-mode socket that can be used with both IPv4 and IPv6.
    ///
    /// If a socket is created using the [`WSASocket`] function, then the `flags` parameter must
    /// have the [`WSA_FLAG_OVERLAPPED`] attribute set for the [`SO_RCVTIMEO`] or [`SO_SNDTIMEO`]
    /// socket options to function properly. Otherwise the timeout never takes effect on the
    /// socket.
    ///
    /// Connection-oriented sockets such as [`SOCK_STREAM`] provide full-duplex connections, and
    /// must be in a connected state before any data can be sent or received on them. A connection
    /// to a specified socket is established with a connect or [`WSAConnect`] function call. Once
    /// connected, data can be transferred using [`send`]/[`WSASend`] and [`recv`]/[`WSARecv`]
    /// calls. When a session has been completed, the [`closesocket`] function should be called to
    /// release the resources associated with the socket. For connection-oriented sockets, the
    /// [`shutdown`] function should be called to stop data transfer on the socket before calling
    /// the [`closesocket`] function.
    ///
    /// The communications protocols used to implement a reliable, connection-oriented socket
    /// ensure that data is not lost or duplicated. If data for which the peer protocol has buffer
    /// space cannot be successfully transmitted within a reasonable length of time, the connection
    /// is considered broken and subsequent calls will fail with the error code set to
    /// [`WSAETIMEDOUT`].
    ///
    /// Connectionless, message-oriented sockets allow sending and receiving of datagrams to and
    /// from arbitrary peers using [`sendto`]/[`WSASendTo`] and [`recvfrom`]/[`WSARecvFrom`]. If
    /// such a socket is connected to a specific peer, datagrams can be sent to that peer using
    /// [`send`]/[`WSASend`] and can be received from (only) this peer using [`recv`]/[`WSARecv`].
    ///
    /// Support for sockets with type [`SOCK_RAW`] is not required, but service providers are
    /// encouraged to support raw sockets whenever possible.
    ///
    /// The [`WSASocket`] function can be used to create a socket to be used by a service so that
    /// if another socket tries to bind to the same port used by the service, and audit record is
    /// generated. To enable this option, an application would need to do the following:
    ///  - Call the [`AdjustTokenPrivileges`] function to enable the `SE_SECURITY_NAME` privilege
    ///    in the access token for the process. This privilege is required to set the
    ///    `ACCESS_SYSTEM_SECURITY` access rights on the security descriptor for an object.
    ///  - Call the [`WSASocket`] function to create a socket with `flag` with the
    ///    [`WSA_FLAG_ACCESS_SYSTEM_SECURITY`] option set. The [`WSASocket`] function will fail if
    ///    the [`AdjustTokenPrivileges`] function is not called first to enable the
    ///    `SE_SECURITY_NAME` privilege needed for this operation.
    ///  - Call the [`SetSecurityInfo`] function to set a security descriptor with a System Access
    ///    Control List (SACL) on the socket. The socket handle returned by the [`WSASocket`]
    ///    function is passed in the handle parameter. If the function succeeds, this will set the
    ///    `ACCESS_SYSTEM_SECURITY` access right on the security descriptor for the socket.
    ///  - Call the [`bind`] function to bind the socket to a specific port. If the [`bind`]
    ///    function succeeds, then an audit entry is generated if another socket tries to bind to
    ///    the same port.
    ///  - Call the [`AdjustTokenPrivileges`] function to remove the `SE_SECURITY_NAME` privilege
    ///    in the access token for the process, since this is no longer needed.
    pub fn WSASocketW(
        af: c_int,
        r#type: c_int,
        protocol: c_int,
        protocol_info: *mut LPWSAPROTOCOL_INFOW,
        g: GROUP,
        flags: DWORD,
    ) -> SOCKET;
}
