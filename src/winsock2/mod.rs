//! Windows socket

mod constants;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use functions::{
    accept, bind, closesocket, connect, gethostname, getsockname, ioctlsocket, listen, recv, send,
    socket, WSACleanup, WSACloseEvent, WSACreateEvent, WSAEventSelect, WSAGetLastError,
    WSAGetOverlappedResult, WSARecv, WSASocket, WSASocketW, WSAStartup,
};
pub use structures::{sockaddr, sockaddr_in, sockaddr_in6, WSABUF, WSADATA, WSAOVERLAPPED};
pub use types::{
    in6_addr, in_addr, GROUP, LPWSABUF, LPWSADATA, LPWSAOVERLAPPED,
    LPWSAOVERLAPPED_COMPLETION_ROUTINE, LPWSAPROTOCOL_INFOW, SOCKET, WSAEVENT,
};
