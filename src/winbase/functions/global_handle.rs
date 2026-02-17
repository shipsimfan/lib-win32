use crate::{HGLOBAL, LPCVOID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, GlobalAlloc, GlobalLock, GMEM_MOVEABLE};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves the handle associated with the specified pointer to a global memory block.
    ///
    /// # Parameters
    ///  * `mem` - A pointer to the first byte of the global memory block. This pointer is returned
    ///            by the [`GlobalLock`] function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the specified global memory
    /// object.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// When the [`GlobalAlloc`] function allocates a memory object with [`GMEM_MOVEABLE`], it
    /// returns a handle to the object. The [`GlobalLock`] function converts this handle into a
    /// pointer to the memory block, and [`GlobalHandle`] converts the pointer back into a handle.
    pub fn GlobalHandle(mem: LPCVOID) -> HGLOBAL;
}
