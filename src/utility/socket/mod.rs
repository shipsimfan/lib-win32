use crate::winsock2::SOCKET;

mod address;
mod event;

mod accept;
mod bind;
mod connect;
mod deref;
mod drop;
mod get;
mod listen;
mod new;
mod recv;
mod send;

pub use address::SocketAddress;
pub use event::SocketEvent;

/// A wrapper around a win32 socket
pub struct Socket {
    /// The handle to the socket
    handle: SOCKET,
}
