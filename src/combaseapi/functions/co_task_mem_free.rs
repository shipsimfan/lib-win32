use crate::LPVOID;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Ole32")]
unsafe extern "system" {
    /// Frees a block of task memory previously allocated through a call to the [`CoTaskMemAlloc`]
    /// or [`CoTaskMemRealloc`] function.
    ///
    /// # Parameters
    ///  * `pv` - A pointer to the memory block to be freed. If this parameter is [`null_mut`], the
    ///           function has no effect.
    ///
    /// # Remarks
    /// The [`CoTaskMemFree`] function uses the default OLE allocator.
    ///
    /// The number of bytes freed equals the number of bytes that were originally allocated or
    /// reallocated. After the call, the memory block pointed to by `pv` is invalid and can no
    /// longer be used.
    pub fn CoTaskMemFree(pv: LPVOID);
}
