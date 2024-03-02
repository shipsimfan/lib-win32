use std::ffi::c_int;

/// The address family is unspecified.
pub const AF_UNSPEC: c_int = 0;

/// The Internet Protocol version 4 (IPv4) address family.
pub const AF_INET: c_int = 2;

/// The IPX/SPX address family.
pub const AF_IPX: c_int = 6;

/// The AppleTalk address family.
pub const AF_APPLETALK: c_int = 16;

/// The NetBIOS address family.
pub const AF_NETBIOS: c_int = 17;

/// The Internet Protocol version 6 (IPv6) address family.
pub const AF_INET6: c_int = 23;

/// The Infrared Data Association (IrDA) address family.
pub const AF_IRDA: c_int = 26;

/// The Bluetooth address family.
pub const AF_BTH: c_int = 32;
