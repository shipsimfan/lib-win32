use crate::{HGLOBAL, SIZE_T, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, GlobalAlloc, GlobalLock, GMEM_MODIFY, GMEM_MOVEABLE, GMEM_ZEROINIT};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Changes the size or attributes of a specified global memory object. The size can increase
    /// or decrease.
    ///
    /// # Parameters
    ///  * `mem` - A handle to the global memory object to be reallocated. This handle is returned
    ///            by either the [`GlobalAlloc`] or [`GlobalReAlloc`] function.
    ///  * `bytes` - The new size of the memory block, in bytes. If `flags` specifies
    ///              [`GMEM_MODIFY`], this parameter is ignored.
    ///  * `flags` - The reallocation options. If [`GMEM_MODIFY`] is specified, the function
    ///              modifies the attributes of the memory object only (the `bytes` parameter is
    ///              ignored.) Otherwise, the function reallocates the memory object. You can
    ///              optionally combine [`GMEM_MODIFY`] with [`GMEM_MOVEABLE`]. If this parameter
    ///              does not specify [`GMEM_MODIFY`], you can use [`GMEM_ZEROINIT`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the reallocated memory object.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// If [`GlobalReAlloc`] reallocates a movable object, the return value is a handle to the
    /// memory object. To convert the handle to a pointer, use the [`GlobalLock`] function.
    ///
    /// If [`GlobalReAlloc`] reallocates a fixed object, the value of the handle returned is the
    /// address of the first byte of the memory block. To access the memory, a process can simply
    /// cast the return value to a pointer.
    ///
    /// If [`GlobalReAlloc`] fails, the original memory is not freed, and the original handle and
    /// pointer are still valid.
    pub fn GlobalReAlloc(mem: HGLOBAL, bytes: SIZE_T, flags: UINT) -> HGLOBAL;
}
