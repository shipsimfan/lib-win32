use crate::winsock2::{sockaddr_in, sockaddr_in6};

mod as_ptr;
mod family;
mod from;
mod into;
mod len;

/// A win32 socket address
#[derive(Debug, Clone)]
pub enum SocketAddress {
    /// The address is for IPv4
    V4(sockaddr_in),

    /// The address is for IPv6
    V6(sockaddr_in6),
}
