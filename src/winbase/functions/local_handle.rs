use crate::{HLOCAL, LPCVOID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, LocalAlloc, LocalLock, LMEM_MOVEABLE};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Retrieves the handle associated with the specified pointer to a local memory object.
    ///
    /// # Parameters
    ///  * `mem` - A pointer to the first byte of the local memory object. This pointer is returned
    ///            by the [`LocalLock`] function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the specified local memory
    /// object.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// When the [`LocalAlloc`] function allocates a local memory object with [`LMEM_MOVEABLE`], it
    /// returns a handle to the object. The [`LocalLock`] function converts this handle into a
    /// pointer to the object's memory block, and [`LocalHandle`] converts the pointer back into a
    /// handle.
    pub fn LocalHandle(mem: LPCVOID) -> HLOCAL;
}
