use std::ffi::c_int;

/// The address family is unspecified.
pub const AF_UNSPEC: c_int = 0;

/// The Internet Protocol version 4 (IPv4) address family.
pub const AF_INET: c_int = 2;

/// The NetBIOS address family. This address family is only supported if a
/// Windows Sockets provider for NetBIOS is installed.
pub const AF_NETBIOS: c_int = 17;

/// The Internet Protocol version 6 (IPv6) address family.
pub const AF_INET6: c_int = 23;

/// The Infrared Data Association (IrDA) address family. This address family
/// is only supported if the computer has an infrared port and driver
/// installed.
pub const AF_IRDA: c_int = 26;

/// The Bluetooth address family. This address family is only supported if a
/// Bluetooth adapter is installed on Windows Server 2003 or later.
pub const AF_BTH: c_int = 32;
