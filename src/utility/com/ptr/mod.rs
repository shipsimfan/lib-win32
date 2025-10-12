use crate::unknwn::IUnknownTrait;
use std::ptr::NonNull;

mod clone;
mod deref;
mod drop;
mod new;
mod new_in;
mod query_interface;

/// A pointer to a COM object
pub struct ComPtr<T: IUnknownTrait> {
    /// The pointer to the interface
    ptr: NonNull<T>,
}
