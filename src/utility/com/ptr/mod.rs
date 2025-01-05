use super::COMInterface;
use std::ptr::NonNull;

mod clone;
mod deref;
mod drop;
mod new;
mod query_interface;

/// A pointer to a COM object
pub struct COMPtr<T: COMInterface> {
    /// The pointer to the interface
    ptr: NonNull<T>,
}
