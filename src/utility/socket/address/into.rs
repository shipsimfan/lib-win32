use crate::SocketAddress;
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

impl Into<SocketAddr> for SocketAddress {
    fn into(self) -> SocketAddr {
        match self {
            SocketAddress::V4(addr) => SocketAddr::V4(SocketAddrV4::new(
                Ipv4Addr::from_bits(u32::from_be(addr.addr)),
                u16::from_be_bytes(addr.port.to_ne_bytes()),
            )),
            SocketAddress::V6(addr) => SocketAddr::V6(SocketAddrV6::new(
                Ipv6Addr::from_bits(u128::from_be_bytes(addr.addr)),
                u16::from_be_bytes(addr.port.to_ne_bytes()),
                u32::from_be_bytes(addr.flowinfo.to_ne_bytes()),
                u32::from_be_bytes(addr.scope_id.to_ne_bytes()),
            )),
        }
    }
}
