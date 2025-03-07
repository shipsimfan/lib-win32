use crate::winsock2::SOCKET;

mod address;

mod deref;
mod drop;
mod get;
mod new;

pub use address::SocketAddress;

/// A wrapper around a win32 socket
pub struct Socket {
    /// The handle to the socket
    handle: SOCKET,
}
