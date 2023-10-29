use crate::raw::{sockaddr, PWStr};
use core::ffi::{c_int, c_size_t};

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{
    bind, GetAddrInfoW, AF_BTH, AF_INET, AF_INET6, AF_IRDA, AF_NETBIOS, AF_UNSPEC, AI_ADDRCONFIG,
    AI_ALL, AI_CANONNAME, AI_DISABLE_IDN_ENCODING, AI_FILESERVER, AI_FQDN, AI_NON_AUTHORITATIVE,
    AI_NUMERICHOST, AI_PASSIVE, AI_RETURN_PREFERRED_NAMES, AI_SECURE, AI_V4MAPPED, IPPROTO_PGM,
    IPPROTO_RM, IPPROTO_TCP, IPPROTO_UDP, SOCK_DGRAM, SOCK_RAW, SOCK_RDM, SOCK_SEQPACKET,
    SOCK_STREAM,
};
#[allow(unused_imports)]
use std::ptr::null;

pub type PAddrInfoW = *mut AddrInfoW;

/// # AddrInfoW structure (ws2def.h)
///
/// The [`AddrInfoW`] structure is used by the [`GetAddrInfoW`] function to
/// hold host address information.
///
/// ## Members
/// `flags`\
/// Flags that indicate options used in the [`GetAddrInfoW`] function.
///
/// Supported values for the `flags` member are defined in the Winsock2.h
/// header file and can be a combination of the options listed in the
/// following table.
///
/// | Value                         | Meaning                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
/// |-------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`AI_PASSIVE`]                | The socket address will be used in a call to the [`bind`] function.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
/// | [`AI_CANONNAME`]              | The canonical name is returned in the first `canon_name` member.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
/// | [`AI_NUMERICHOST`]            | The nodename parameter passed to the [`GetAddrInfoW`] function must be a numeric string.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
/// | [`AI_ALL`]                    | If this bit is set, a request is made for IPv6 addresses and IPv4 addresses with [`AI_V4MAPPED`].                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          |
/// | [`AI_ADDRCONFIG`]             | The [`GetAddrInfoW`] will resolve only if a global address is configured. The IPv6 and IPv4 loopback address is not considered a valid global address. This option is only supported on Windows Vista and later.                                                                                                                                                                                                                                                                                                                                                                                                                                           |
/// | [`AI_V4MAPPED`]               | If the [`GetAddrInfoW`] request for an IPv6 addresses fails, a name service request is made for IPv4 addresses and these addresses are converted to IPv4-mapped IPv6 address format.                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
/// | [`AI_NON_AUTHORITATIVE`]      | The address information can be from a non-authoritative namespace provider. This option is only supported on Windows Vista and later for the [`NS_EMAIL`] namespace.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
/// | [`AI_SECURE`]                 | The address information is from a secure channel. This option is only supported on Windows Vista and later for the [`NS_EMAIL`] namespace.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
/// | [`AI_RETURN_PREFERRED_NAMES`] | The address information is for a preferred name for a user. This option is only supported on Windows Vista and later for the [`NS_EMAIL`] namespace.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                       |
/// | [`AI_FQDN`]                   | If a flat name (single label) is specified, [`GetAddrInfoW`] will return the fully qualified domain name that the name eventually resolved to. The fully qualified domain name is returned in the `canon_name` member. This is different than [`AI_CANONNAME`] bit flag that returns the canonical name registered in DNS which may be different than the fully qualified domain name that the flat name resolved to. Only one of the [`AI_FQDN`] and [`AI_CANONNAME`] bits can be set. The [`GetAddrInfoW`] function will fail if both flags are present with [`EAI_BADFLAGS`]. This option is supported on Windows 7, Windows Server 2008 R2, and later. |
/// | [`AI_FILESERVER`]             | A hint to the namespace provider that the hostname being queried is being used in a file share scenario. The namespace provider may ignore this hint. This option is supported on Windows 7, Windows Server 2008 R2, and later.                                                                                                                                                                                                                                                                                                                                                                                                                            |
/// | [`AI_DISABLE_IDN_ENCODING`]   | Disable the automatic International Domain Name encoding using Punycode in the name resolution functions called by the [`GetAddrInfoW`] function. This option is supported on Windows 8, Windows Server 2012, and later.                                                                                                                                                                                                                                                                                                                                                                                                                                   |
///
/// `family`\
/// The address family. Possible values for the address family are defined in
/// the Winsock2.h header file.
///
/// On the Windows SDK released for Windows Vista and later, the organization
/// of header files has changed and the possible values for the address family
/// are defined in the Ws2def.h header file. Note that the Ws2def.h header file
/// is automatically included in Winsock2.h, and should never be used directly.
///
/// The values currently supported are AF_INET or AF_INET6, which are the
/// Internet address family formats for IPv4 and IPv6. Other options for address
/// family (AF_NETBIOS for use with NetBIOS, for example) are supported if a
/// Windows Sockets service provider for the address family is installed. Note
/// that the values for the AF_ address family and PF_ protocol family
/// constants are identical (for example, AF_UNSPEC and PF_UNSPEC), so either
/// constant can be used.
///
/// The following table lists common values for the address family although
/// many other values are possible.
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
/// `sock_type`
/// The socket type. Possible values for the socket type are defined in the
/// Winsock2.h include file.
///
/// The following table lists the possible values for the socket type supported
/// for Windows Sockets 2.
///
/// | Value              | Meaning                                                                                                                                                                                                                                                                                                                  |
/// |--------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`SOCK_STREAM`]    | Provides sequenced, reliable, two-way, connection-based byte streams with an OOB data transmission mechanism. Uses the Transmission Control Protocol (TCP) for the Internet address family ([`AF_INET`] or [`AF_INET6`]). If the `family` member is [`AF_IRDA`], then [`SOCK_STREAM`] is the only supported socket type. |
/// | [`SOCK_DGRAM`]     | Supports datagrams, which are connectionless, unreliable buffers of a fixed (typically small) maximum length. Uses the User Datagram Protocol (UDP) for the Internet address family ([`AF_INET`] or [`AF_INET6`]).                                                                                                       |
/// | [`SOCK_RAW`]       | Provides a raw socket that allows an application to manipulate the next upper-layer protocol header. To manipulate the IPv4 header, the [`IP_HDRINCL`] socket option must be set on the socket. To manipulate the IPv6 header, the [`IPV6_HDRINCL`] socket option must be set on the socket.                             |
/// | [`SOCK_RDM`]       | Provides a reliable message datagram. An example of this type is the Pragmatic General Multicast (PGM) multicast protocol implementation in Windows, often referred to as reliable multicast programming.                                                                                                                |
/// | [`SOCK_SEQPACKET`] | Provides a pseudo-stream packet based on datagrams.                                                                                                                                                                                                                                                                      |
///
/// In Windows Sockets 2, new socket types were introduced. An application can
/// dynamically discover the attributes of each available transport protocol
/// through the [`WSAEnumProtocols`] function. So an application can determine
/// the possible socket type and protocol options for an address family and use
/// this information when specifying this parameter. Socket type definitions in
/// the Winsock2.h and Ws2def.h header files will be periodically updated as
/// new socket types, address families, and protocols are defined.
///
/// In Windows Sockets 1.1, the only possible socket types are
/// [`SOCK_DGRAM`] and [`SOCK_STREAM`].
///
/// `protocol`\
/// The protocol type. The possible options are specific to the address family
/// and socket type specified. Possible values for the `protocol` are defined
/// in Winsock2.h and the Wsrm.h header files.
///
/// On the Windows SDK released for Windows Vista and later, the organization
/// of header files has changed and this member can be one of the values from
/// the [`IPPROTO`] enumeration type defined in the Ws2def.h header file. Note
/// that the Ws2def.h header file is automatically included in Winsock2.h, and
/// should never be used directly.
///
/// If a value of 0 is specified for `protocol`, the caller does not wish to
/// specify a protocol and the service provider will choose the `protocol` to
/// use. For protocols other than IPv4 and IPv6, set `protocol` to zero.
///
/// The following table lists common values for the `protocol` member although
/// many other values are possible.
///
/// | Value           | Meaning                                                                                                                                                                                                                                                  |
/// |-----------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | [`IPPROTO_TCP`] | The Transmission Control Protocol (TCP). This is a possible value when the `family` member is [`AF_INET`] or [`AF_INET6`] and the `socktype` member is [`SOCK_STREAM`].                                                                                  |
/// | [`IPPROTO_UDP`] | The User Datagram Protocol (UDP). This is a possible value when the `family` member is [`AF_INET`] or [`AF_INET6`] and the type parameter is [`SOCK_DGRAM`].                                                                                             |
/// | [`IPPROTO_RM`]  | The PGM protocol for reliable multicast. This is a possible value when the `family` member is [`AF_INET`] and the `socktype` member is [`SOCK_RDM`]. On the Windows SDK released for Windows Vista and later, this value is also called [`IPPROTO_PGM`]. |
///
/// `addr_len`\
/// The length, in bytes, of the buffer pointed to by the `addr` member.
///
/// `canon_name`\
/// The canonical name for the host.
///
/// `addr`\
/// A pointer to a sockaddr structure. The `addr` member in each returned
/// [`AddrInfoW`] structure points to a filled-in socket address structure. The
/// length, in bytes, of each returned [`AddrInfoW`] structure is specified in
/// the `addr_len` member.
///
/// `next`\
/// A pointer to the next structure in a linked list. This parameter is set to
/// [`null`] in the last [`AddrInfoW`] structure of a linked list.
///
/// ## Remarks
/// The [`AddrInfoW`] structure is used by the Unicode [`GetAddrInfoW`] function to
/// hold host address information.
///
/// Upon a successful call to [`GetAddrInfoW`], a linked list of [`AddrInfoW`]
/// structures is returned in the `result` parameter passed to the
/// [`GetAddrInfoW`] function. The list can be processed by following the
/// pointer provided in the `next` member of each returned [`AddrInfoW`]
/// structure until a [`null`] pointer is encountered. In each returned
/// [`AddrInfoW`] structure, the `family`, `sock_type`, and `protocol` members
/// correspond to respective arguments in a [`socket`] or [`WSASocket`]
/// function call. Also, the `addr` member in each returned [`AddrInfoW`]
/// structure points to a filled-in socket address structure, the length of
/// which is specified in its `addr_len` member.
#[repr(C)]
#[derive(Clone)]
pub struct AddrInfoW {
    pub flags: c_int,
    pub family: c_int,
    pub sock_type: c_int,
    pub protocol: c_int,
    pub addr_len: c_size_t,
    pub canon_name: PWStr,
    pub addr: *mut sockaddr,
    pub next: *mut AddrInfoW,
}

impl AddrInfoW {
    pub fn new() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
