use crate::winsock2::SOCKET;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    winsock2::{
        AF_APPLETALK, AF_BTH, AF_INET, AF_INET6, AF_IPX, AF_IRDA, AF_NETBIOS, AF_UNSPEC,
        BTHPROTO_RFCOMM, INVALID_SOCKET, IPPROTO_ICMP, IPPROTO_ICMPV6, IPPROTO_IGMP, IPPROTO_RM,
        IPPROTO_TCP, IPPROTO_UDP, SOCK_DGRAM, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET, SOCK_STREAM,
    },
    WSAEAFNOSUPPORT, WSAEPROTONOSUPPORT, WSAETIMEDOUT,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Ws2_32")]
extern "system" {
    /// The socket function creates a socket that is bound to a specific transport service provider.
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
    ///
    /// # Return Value
    /// If no error occurs, [`socket`] returns a descriptor referencing the new socket. Otherwise,
    /// a value of [`INVALID_SOCKET`] is returned, and a specific error code can be retrieved by
    /// calling [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`socket`] function causes a socket descriptor and any related resources to be
    /// allocated and bound to a specific transport-service provider. Winsock will utilize the
    /// first available service provider that supports the requested combination of address family,
    /// socket type and protocol parameters. The socket that is created will have the overlapped
    /// attribute as a default. For Windows, the Microsoft-specific socket option, [`SO_OPENTYPE`],
    /// can affect this default.
    ///
    /// Sockets without the overlapped attribute can be created by using [`WSASocket`]. All
    /// functions that allow overlapped operation ([`WSASend`], [`WSARecv`], [`WSASendTo`],
    /// [`WSARecvFrom`], and [`WSAIoctl`]) also support nonoverlapped usage on an overlapped socket
    /// if the values for parameters related to overlapped operation are [`null`].
    ///
    /// When selecting a protocol and its supporting service provider this procedure will only
    /// choose a base protocol or a protocol chain, not a protocol layer by itself. Unchained
    /// protocol layers are not considered to have partial matches on type or af either. That is,
    /// they do not lead to an error code of [`WSAEAFNOSUPPORT`] or [`WSAEPROTONOSUPPORT`] if no
    /// suitable protocol is found.
    ///
    /// Applications are encouraged to use [`AF_INET6`] for the af parameter and create a dual-mode
    /// socket that can be used with both IPv4 and IPv6.
    ///
    /// Connection-oriented sockets such as [`SOCK_STREAM`] provide full-duplex connections, and
    /// must be in a connected state before any data can be sent or received on it. A connection to
    /// another socket is created with a connect call. Once connected, data can be transferred
    /// using [`send`] and [`recv`] calls. When a session has been completed, a closesocket must be
    /// performed.
    ///
    /// The communications protocols used to implement a reliable, connection-oriented socket
    /// ensure that data is not lost or duplicated. If data for which the peer protocol has buffer
    /// space cannot be successfully transmitted within a reasonable length of time, the connection
    /// is considered broken and subsequent calls will fail with the error code set to
    /// [`WSAETIMEDOUT`].
    ///
    /// Connectionless, message-oriented sockets allow sending and receiving of datagrams to and
    /// from arbitrary peers using [`sendto`] and [`recvfrom`]. If such a socket is connected to a
    /// specific peer, datagrams can be sent to that peer using [`send`] and can be received only
    /// from this peer using [`recv`].
    ///
    /// IPv6 and IPv4 operate differently when receiving a socket with a type of [`SOCK_RAW`]. The
    /// IPv4 receive packet includes the packet payload, the next upper-level header (for example,
    /// the IP header for a TCP or UDP packet), and the IPv4 packet header. The IPv6 receive packet
    /// includes the packet payload and the next upper-level header. The IPv6 receive packet never
    /// includes the IPv6 packet header.
    ///
    /// A socket with a type parameter of [`SOCK_SEQPACKET`] is based on datagrams, but functions
    /// as a pseudo-stream protocol. For both send and receive packets, separate datagrams are
    /// used. However, Windows Sockets can coalesce multiple receive packets into a single packet.
    /// So an application can issue a receive call (for example, [`recv`] or [`WSARecvEx`]) and
    /// retrieve the data from several coalesced multiple packets in single call. The
    /// [`AF_NETBIOS`] address family supports a type parameter of [`SOCK_SEQPACKET`].
    ///
    /// When the `af` parameter is [`AF_NETBIOS`] for NetBIOS over TCP/IP, the type parameter can
    /// be [`SOCK_DGRAM`] or [`SOCK_SEQPACKET`]. For the [`AF_NETBIOS`] address family, the
    /// protocol parameter is the LAN adapter number represented as a negative number.
    pub fn socket(af: c_int, r#type: c_int, protocol: c_int) -> SOCKET;
}
