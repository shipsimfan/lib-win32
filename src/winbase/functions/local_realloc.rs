use crate::{HLOCAL, SIZE_T, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, LocalAlloc, LMEM_MODIFY, LMEM_MOVEABLE, LMEM_ZEROINIT};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Changes the size or the attributes of a specified local memory object. The size can
    /// increase or decrease.
    ///
    /// # Parameters
    ///  * `mem` - A handle to the local memory object to be reallocated. This handle is returned
    ///            by either the [`LocalAlloc`] or [`LocalReAlloc`] function.
    ///  * `bytes` - The new size of the memory block, in bytes. If `flags` specifies
    ///              [`LMEM_MODIFY`], this parameter is ignored.
    ///  * `flags` - The reallocation options. If [`LMEM_MODIFY`] is specified, the function
    ///              modifies the attributes of the memory object only (the `bytes` parameter is
    ///              ignored.) Otherwise, the function reallocates the memory object. You can
    ///              optionally combine [`LMEM_MODIFY`] with [`LMEM_MOVEABLE`]. If this parameter
    ///              does not specify [`LMEM_MODIFY`], you can use [`LMEM_ZEROINIT`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the reallocated memory object.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// If [`LocalReAlloc`] fails, the original memory is not freed, and the original handle and
    /// pointer are still valid.
    ///
    /// If [`LocalReAlloc`] reallocates a fixed object, the value of the handle returned is the
    /// address of the first byte of the memory block. To access the memory, a process can simply
    /// cast the return value to a pointer.
    pub fn LocalReAlloc(mem: HLOCAL, bytes: SIZE_T, flags: UINT) -> HLOCAL;
}
