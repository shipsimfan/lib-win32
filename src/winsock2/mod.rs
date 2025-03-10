//! Windows socket

mod constants;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use functions::{
    accept, bind, closesocket, connect, getsockname, ioctlsocket, listen, recv, send, socket,
    WSACleanup, WSACloseEvent, WSACreateEvent, WSAEventSelect, WSAGetLastError, WSASocket,
    WSASocketW, WSAStartup,
};
pub use structures::{sockaddr, sockaddr_in, sockaddr_in6, WSADATA};
pub use types::{in6_addr, in_addr, GROUP, LPWSADATA, LPWSAPROTOCOL_INFOW, SOCKET, WSAEVENT};
