use crate::winsock2::WSAEVENT;

mod deref;
mod drop;
mod get;
mod new;
mod select;
mod wait;

/// An event that is signalled by one or more sockets
pub struct SocketEvent {
    /// The handle to the underlying event
    handle: WSAEVENT,
}

unsafe impl Send for SocketEvent {}
unsafe impl Sync for SocketEvent {}
