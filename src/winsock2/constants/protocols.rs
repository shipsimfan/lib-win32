use std::ffi::c_int;

/// The Internet Control Message Protocol (ICMP)
pub const IPPROTO_ICMP: c_int = 1;

/// The Internet Group Management Protocol (IGMP)
pub const IPPROTO_IGMP: c_int = 2;

/// The Bluetooth Radio Frequency Communications (Bluetooth RFCOMM)
pub const BTHPROTO_RFCOMM: c_int = 3;

/// The Transmission Control Protocol (TCP)
pub const IPPROTO_TCP: c_int = 6;

/// The User Datagram Protocol (UDP)
pub const IPPROTO_UDP: c_int = 17;

/// The Internet Control Message Protocol Version 6 (ICMPv6)
pub const IPPROTO_ICMPV6: c_int = 58;

/// The PGM protocol for reliable multicast
pub const IPPROTO_RM: c_int = 113;
