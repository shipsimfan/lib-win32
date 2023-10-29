use crate::raw::{DWord, Group, LPWSAProtocolInfoW, Socket};
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{
    WSAGetLastError, WSAProtocolInfoW, AF_INET, AF_INET6, AF_NETBIOS, INVALID_SOCKET, SOCK_RAW,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Ws2_32")]
extern "C" {
    /// # WSASocketW function (winsock2.h)
    ///
    /// The [`WSASocketW`] function creates a socket that is bound to a specific transport-service provider.
    ///
    /// ## Parameters
    /// `af`\
    /// The address family specification. Possible values for the address
    /// family are defined in the Winsock2.h header file.
    ///
    /// On the Windows SDK released for Windows Vista and later, the
    /// organization of header files has changed and the possible values for
    /// the address family are defined in the Ws2def.h header file. Note that
    /// the Ws2def.h header file is automatically included in Winsock2.h, and
    /// should never be used directly.
    ///
    /// The values currently supported are [`AF_INET`] or [`AF_INET6`], which
    /// are the Internet address family formats for IPv4 and IPv6. Other
    /// options for address family ([`AF_NETBIOS`] for use with NetBIOS, for
    /// example) are supported if a Windows Sockets service provider for the
    /// address family is installed. Note that the values for the AF_ address
    ///  family and PF_ protocol family constants are identical (for example,
    /// [`AF_INET`] and [`PF_INET`]), so either constant can be used.
    ///
    /// `r#type`\
    /// The type specification for the new socket.
    ///
    /// Possible values for the socket type are defined in the Winsock2.h
    /// header file.
    ///
    /// In Windows Sockets 2, new socket types were introduced. An application
    /// can dynamically discover the attributes of each available transport
    /// protocol through the [`WSAEnumProtocols`] function. So an application
    /// can determine the possible socket type and protocol options for an
    /// address family and use this information when specifying this parameter.
    /// Socket type definitions in the Winsock2.h and Ws2def.h header files
    /// will be periodically updated as new socket types, address families, and
    /// protocols are defined.
    ///
    /// `protocol`\
    /// The protocol to be used. The possible options for the protocol
    /// parameter are specific to the address family and socket type specified.
    /// Possible values for the protocol are defined are defined in the
    /// Winsock2.h and Wsrm.h header files.
    ///
    /// On the Windows SDK released for Windows Vista and later, the
    /// organization of header files has changed and this parameter can be one
    /// of the values from the IPPROTO enumeration type defined in the Ws2def.h
    /// header file. Note that the Ws2def.h header file is automatically
    /// included in Winsock2.h, and should never be used directly.
    ///
    /// If a value of 0 is specified, the caller does not wish to specify a
    /// protocol and the service provider will choose the protocol to use.
    ///
    /// When the `af` parameter is [`AF_INET`] or [`AF_INET6`] and the type is
    /// [`SOCK_RAW`], the value specified for the protocol is set in the
    /// protocol field of the IPv6 or IPv4 packet header.
    ///
    /// `protocol_info`\
    /// A pointer to a [`WSAProtocolInfoW`] structure that defines the
    /// characteristics of the socket to be created. If this parameter is not
    /// [`null`], the socket will be bound to the provider associated with the
    /// indicated [`WSAProtocolInfoW`] structure.
    ///
    /// `g`\
    /// An existing socket group ID or an appropriate action to take when
    /// creating a new socket and a new socket group.
    ///
    /// If g is an existing socket group ID, join the new socket to this socket
    /// group, provided all the requirements set by this group are met.
    ///
    /// `flags`\
    /// A set of flags used to specify additional socket attributes.
    ///
    /// ## Return Value
    /// If no error occurs, [`WSASocketW`] returns a descriptor referencing the
    /// new socket. Otherwise, a value of [`INVALID_SOCKET`] is returned, and a
    /// specific error code can be retrieved by calling [`WSAGetLastError`].
    pub fn WSASocketW(
        af: c_int,
        r#type: c_int,
        protocol: c_int,
        protocol_info: LPWSAProtocolInfoW,
        g: Group,
        flags: DWord,
    ) -> Socket;
}
