use crate::{
    SocketAddress,
    winsock2::{sockaddr_in, sockaddr_in6, AF_INET, AF_INET6},
};
use std::net::{SocketAddr, SocketAddrV4, SocketAddrV6};

impl From<SocketAddr> for SocketAddress {
    fn from(addr: SocketAddr) -> Self {
        match addr {
            SocketAddr::V4(addr) => addr.into(),
            SocketAddr::V6(addr) => addr.into(),
        }
    }
}

impl From<SocketAddrV4> for SocketAddress {
    fn from(addr: SocketAddrV4) -> Self {
        SocketAddress::V4(sockaddr_in {
            family: AF_INET as _,
            port: addr.port().to_be(),
            addr: u32::from_ne_bytes(addr.ip().octets()),
            zero: [0; 8],
        })
    }
}

impl From<SocketAddrV6> for SocketAddress {
    fn from(addr: SocketAddrV6) -> Self {
        SocketAddress::V6(sockaddr_in6 {
            family: AF_INET6 as _,
            port: addr.port().to_be(),
            flowinfo: addr.flowinfo().to_be(),
            addr: addr.ip().octets(),
            scope_id: addr.scope_id().to_be(),
        })
    }
}
