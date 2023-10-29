use crate::raw::{DWord, WChar, WSAProtocolChain, GUID};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{
    sockaddr, WSASocketW, AF_BTH, AF_INET, AF_INET6, AF_IRDA, AF_NETBIOS, AF_UNSPEC, IPPROTO_RM,
    IPPROTO_TCP, IPPROTO_UDP, SOCK_DGRAM, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET, SOCK_STREAM,
};

/// # WSAProtocolInfoW structure (winsock2.h)
///
/// The [`WSAProtocolInfoW`] structure is used to store or retrieve complete
/// information for a given protocol. The protocol name is represented as an
/// array of Unicode characters.
///
/// ## Members
/// `service_flags_1`\
/// A bitmask that describes the services provided by the protocol. The
/// possible values for this member are defined in the Winsock2.h header file.
///
/// `service_flags_2`\
/// Reserved for additional protocol-attribute definitions.
///
/// `service_flags_3`\
/// Reserved for additional protocol-attribute definitions.
///
/// `service_flags_4`\
/// Reserved for additional protocol-attribute definitions.
///
/// `provider_flags`\
/// A set of flags that provides information on how this protocol is
/// represented in the Winsock catalog. The possible values for this member are
/// defined in the Winsock2.h header file.
///
/// `provider_id`\
/// A globally unique identifier (GUID) assigned to the provider by the service
/// provider vendor. This value is useful for instances where more than one
/// service provider is able to implement a particular protocol. An application
/// can use the ProviderId member to distinguish between providers that might
/// otherwise be indistinguishable.
///
/// `catalog_entry_id`\
/// A unique identifier assigned by the WS2_32.DLL for each
/// [`WSAProtocolInfoW`] structure.
///
/// `protocol_chain`\
/// The [`WSAProtocolChain`] structure associated with the protocol. If the
/// length of the chain is 0, this [`WSAProtocolInfoW`] entry represents a
/// layered protocol which has Windows Sockets 2 SPI as both its top and bottom
/// edges. If the length of the chain equals 1, this entry represents a base
/// protocol whose Catalog Entry identifier is in the dwCatalogEntryId member
/// of the [`WSAProtocolInfoW`] structure. If the length of the chain is larger
/// than 1, this entry represents a protocol chain which consists of one or
/// more layered protocols on top of a base protocol. The corresponding Catalog
/// Entry identifiers are in the ProtocolChain.ChainEntries array starting with
/// the layered protocol at the top (the zero element in the
/// ProtocolChain.ChainEntries array) and ending with the base protocol. Refer
/// to the Windows Sockets 2 Service Provider Interface specification for more
/// information on protocol chains.
///
/// `version`
/// The protocol version identifier.
///
/// `address_family`
/// A value to pass as the address family parameter to the [`socket`] or
/// [`WSASocketW`] function in order to open a socket for this protocol. This
/// value also uniquely defines the structure of a protocol address for a
/// [`sockaddr`] used by the protocol.
///
/// On the Windows SDK released for Windows Vista and later, the possible
/// values for the address family are defined in the Ws2def.h header file. Note
/// that the Ws2def.h header file is automatically included in Winsock2.h, and
/// should never be used directly.
///
/// On versions of the Platform SDK for Windows Server 2003 and older, the
/// possible values for the address family are defined in the Winsock2.h header
/// file.
///
/// The values currently supported are [`AF_INET`] or [`AF_INET6`], which are
/// the Internet address family formats for IPv4 and IPv6. Other options for
/// address family ([`AF_NETBIOS`] for use with NetBIOS, for example) are
/// supported if a Windows Sockets service provider for the address family is
/// installed. Note that the values for the AF_ address family and PF_ protocol
/// family constants are identical (for example, [`AF_INET`] and [`PF_INET`]),
/// so either constant can be used.
///
/// The table below lists common values for address family although many other
/// values are possible.
///
/// | Value          | Meaning                                                                                                                                               |
/// |----------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`AF_UNSPEC`]  | The address family is unspecified.                                                                                                                    |
/// | [`AF_INET`]    | The Internet Protocol version 4 (IPv4) address family.                                                                                                |
/// | [`AF_NETBIOS`] | The NetBIOS address family. This address family is only supported if a Windows Sockets provider for NetBIOS is installed.                             |
/// | [`AF_INET6`]   | The Internet Protocol version 6 (IPv6) address family.                                                                                                |
/// | [`AF_IRDA`]    | The Infrared Data Association (IrDA) address family. This address family is only supported if the computer has an infrared port and driver installed. |
/// | [`AF_BTH`]     | The Bluetooth address family. This address family is only supported if a Bluetooth adapter is installed on Windows Server 2003 or later.              |
///
/// `max_sock_addr`\
/// The maximum address length, in bytes.
///
/// `min_sock_addr`\
/// The minimum address length, in bytes.
///
/// `socket_type`\
/// A value to pass as the socket type parameter to the [`socket`] or
/// [`WSASocketW`] function in order to open a socket for this protocol.
/// Possible values for the socket type are defined in the Winsock2.h header
/// file.
///
/// The following table lists the possible values for the `socket_type` member
/// supported for Windows Sockets 2:
///
/// | Value              | Meaning                                                                                                                                                                                                                                                                                                                  |
/// |--------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`SOCK_STREAM`]    | Provides sequenced, reliable, two-way, connection-based byte streams with an OOB data transmission mechanism. Uses the Transmission Control Protocol (TCP) for the Internet address family ([`AF_INET`] or [`AF_INET6`]). If the `family` member is [`AF_IRDA`], then [`SOCK_STREAM`] is the only supported socket type. |
/// | [`SOCK_DGRAM`]     | Supports datagrams, which are connectionless, unreliable buffers of a fixed (typically small) maximum length. Uses the User Datagram Protocol (UDP) for the Internet address family ([`AF_INET`] or [`AF_INET6`]).                                                                                                       |
/// | [`SOCK_RAW`]       | Provides a raw socket that allows an application to manipulate the next upper-layer protocol header. To manipulate the IPv4 header, the [`IP_HDRINCL`] socket option must be set on the socket. To manipulate the IPv6 header, the [`IPV6_HDRINCL`] socket option must be set on the socket.                             |
/// | [`SOCK_RDM`]       | Provides a reliable message datagram. An example of this type is the Pragmatic General Multicast (PGM) multicast protocol implementation in Windows, often referred to as reliable multicast programming.                                                                                                                |
/// | [`SOCK_SEQPACKET`] | Provides a pseudo-stream packet based on datagrams.                                                                                                                                                                                                                                                                      |
///
/// `protocol`\
/// A value to pass as the protocol parameter to the [`socket`] or
/// [`WSASocketW`] function in order to open a socket for this protocol. The
/// possible options for the `protocol` member are specific to the address
/// family and socket type specified.
///
/// On the Windows SDK released for Windows Vista and later, this member can be
/// one of the values from the [`IPPROTO`] enumeration type defined in the
/// Ws2def.h header file. Note that the Ws2def.h header file is automatically
/// included in Winsock2.h, and should never be used directly.
///
/// On versions of the Platform SDK for Windows Server 2003 and earlier, the
/// possible values for the `protocol` member are defined in the Winsock2.h and
/// Wsrm.h header files.
///
/// The table below lists common values for the `protocol` although many other
/// values are possible.
///
/// | Value           | Meaning                                                                                                                                                                                                                                                  |
/// |-----------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`IPPROTO_TCP`] | The Transmission Control Protocol (TCP). This is a possible value when the `family` member is [`AF_INET`] or [`AF_INET6`] and the `socktype` member is [`SOCK_STREAM`].                                                                                  |
/// | [`IPPROTO_UDP`] | The User Datagram Protocol (UDP). This is a possible value when the `family` member is [`AF_INET`] or [`AF_INET6`] and the type parameter is [`SOCK_DGRAM`].                                                                                             |
/// | [`IPPROTO_RM`]  | The PGM protocol for reliable multicast. This is a possible value when the `family` member is [`AF_INET`] and the `socktype` member is [`SOCK_RDM`]. On the Windows SDK released for Windows Vista and later, this value is also called [`IPPROTO_PGM`]. |
///
/// `protocol_max_offset`\
/// The maximum value that may be added to `protocol` member when supplying a
/// value for the protocol parameter to [`socket`] and [`WSASocketW`]. Not all
/// protocols allow a range of values. When this is the case iProtocolMaxOffset
/// is zero.
///
/// `network_byte_order`\
/// Currently these values are manifest constants ([`BIGENDIAN`] and
/// [`LITTLEENDIAN`]) that indicate either big-endian or little-endian with the
/// values 0 and 1 respectively.
///
/// `security_scheme`\
/// The type of security scheme employed (if any). A value of
/// [`SECURITY_PROTOCOL_NONE`] (0) is used for protocols that do not
/// incorporate security provisions.
///
/// `message_size`\
/// The maximum message size, in bytes, supported by the protocol. This is the
/// maximum size that can be sent from any of the host's local interfaces. For
/// protocols that do not support message framing, the actual maximum that can
/// be sent to a given address may be less. There is no standard provision to
/// determine the maximum inbound message size.
///
/// `provider_reserved`\
/// Reserved for use by service providers.
///
/// `sz_protocol`\
/// An array of Unicode characters that contains a human-readable name
/// identifying the protocol, for example "MSAFD Tcpip [UDP/IP]". The maximum
/// number of characters allowed is [`WSAPROTOCOL_LEN`], which is defined to be
/// 255.
#[repr(C)]
#[derive(Clone)]
pub struct WSAProtocolInfoW {
    pub service_flags_1: DWord,
    pub service_flags_2: DWord,
    pub service_flags_3: DWord,
    pub service_flags_4: DWord,
    pub provider_flags: DWord,
    pub provider_id: GUID,
    pub catalog_entry_id: DWord,
    pub protocol_chain: WSAProtocolChain,
    pub version: c_int,
    pub address_family: c_int,
    pub max_sock_addr: c_int,
    pub min_sock_addr: c_int,
    pub socket_type: c_int,
    pub protocol: c_int,
    pub protocol_max_offset: c_int,
    pub network_byte_order: c_int,
    pub security_scheme: c_int,
    pub message_size: DWord,
    pub provider_reserved: DWord,
    pub sz_protocol: [WChar; WSAPROTOCOL_LEN + 1],
}

pub const WSAPROTOCOL_LEN: usize = 255;
