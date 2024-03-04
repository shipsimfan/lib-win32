//! Windows socket

mod constants;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use functions::{bind, closesocket, socket, WSACleanup, WSAGetLastError, WSAStartup};
pub use structures::{sockaddr, sockaddr_in, sockaddr_in6};
pub use types::{in6_addr, in_addr, LPWSADATA, SOCKET};
