use crate::HANDLE;

mod deref;
mod drop;
mod get;
mod new;
mod set;
mod wait;

/// A synchronization object that lets threads signal each other to coordinate actions or wait for specific conditions
pub struct Event {
    /// The handle to the Windows event
    handle: HANDLE,
}
