use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::{AF_INET, AF_INET6};

/// A socket type that provides sequenced, reliable, two-way, connection-based byte streams with an
/// OOB data transmission mechanism. This socket type uses the Transmission Control Protocol (TCP)
/// for the Internet address family ([`AF_INET`] or [`AF_INET6`]).
pub const SOCK_STREAM: c_int = 1;

/// A socket type that supports datagrams, which are connectionless, unreliable buffers of a fixed
/// (typically small) maximum length. This socket type uses the User Datagram Protocol (UDP) for
/// the Internet address family ([`AF_INET`] or [`AF_INET6`]).
pub const SOCK_DGRAM: c_int = 2;

/// A socket type that provides a raw socket that allows an application to manipulate the next
/// upper-layer protocol header. To manipulate the IPv4 header, the [`IP_HDRINCL`] socket option
/// must be set on the socket. To manipulate the IPv6 header, the [`IPV6_HDRINCL`] socket option
/// must be set on the socket.
pub const SOCK_RAW: c_int = 3;

/// A socket type that provides a reliable message datagram. An example of this type is the
/// Pragmatic General Multicast (PGM) multicast protocol implementation in Windows, often referred
/// to as reliable multicast programming.
pub const SOCK_RDM: c_int = 4;

/// A socket type that provides a pseudo-stream packet based on datagrams.
pub const SOCK_SEQPACKET: c_int = 5;
