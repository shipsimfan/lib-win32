use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{AF_INET, AF_INET6, SOCK_DGRAM, SOCK_RDM, SOCK_STREAM};

/// The Transmission Control Protocol (TCP). This is a possible value when the
/// `family` member is [`AF_INET`] or [`AF_INET6`] and the `socktype` member is
/// [`SOCK_STREAM`].
pub const IPPROTO_TCP: c_int = 6;

/// The User Datagram Protocol (UDP). This is a possible value when the
/// `family` member is [`AF_INET`] or [`AF_INET6`] and the type parameter is
/// [`SOCK_DGRAM`].
pub const IPPROTO_UDP: c_int = 17;

/// The PGM protocol for reliable multicast. This is a possible value when the
/// `family` member is [`AF_INET`] and the `socktype` member is [`SOCK_RDM`].
/// On the Windows SDK released for Windows Vista and later, this value is also
/// called [`IPPROTO_PGM`].
pub const IPPROTO_RM: c_int = 113;

/// The PGM protocol for reliable multicast. This is a possible value when the
/// `family` member is [`AF_INET`] and the `socktype` member is [`SOCK_RDM`].
/// On the Windows SDK released for Windows Vista and later, this value is also
/// called [`IPPROTO_PGM`].
pub const IPPROTO_PGM: c_int = IPPROTO_RM;
