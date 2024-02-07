use crate::{HLOCAL, SIZE_T, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{LHND, LMEM_FIXED, LMEM_MOVEABLE, LMEM_ZEROINIT, LPTR, NONZEROLHND, NONZEROLPTR};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Allocates the specified number of bytes from the heap.
    ///
    /// # Parameters
    ///  * `flags` - The memory allocation attributes. The default is the [`LMEM_FIXED`] value.
    ///              This parameter can be one or more of the following values, except for the
    ///              incompatible combinations that are specifically noted:
    ///    * [`LHND`] - Combines [`LMEM_MOVEABLE`] and [`LMEM_ZEROINIT`].
    ///    * [`LMEM_FIXED`] - Allocates fixed memory, The return value is a pointer to the memory
    ///                       object.
    ///    * [`LMEM_MOVEABLE`] - Allocates movable memory. Memory blocks are never moved in physical
    ///                         memory, but they can be moved within the default heap. The return
    ///                         value is a handle to the memory object. To translate the handle to
    ///                         a pointer, use the [`LocalLock`] function. This value cannot be
    ///                         combined with [`LMEM_FIXED`].
    ///    * [`LMEM_ZEROINIT`] - Initializes memory contents to zero.
    ///    * [`LPTR`] - Combines [`LMEM_FIXED`] and [`LMEM_ZEROINIT`].
    ///    * [`NONZEROLHND`] - Same as [`LMEM_MOVEABLE`].
    ///    * [`NONZEROLPTR`] - Same as [`LMEM_FIXED`].
    ///  * `bytes` - The number of bytes to allocate. If this parameter is zero and the `flags`
    ///              parameter specifies [`LMEM_MOVEABLE`], the function returns a handle to a
    ///              memory object that is marked as discarded.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the newly allocated memory
    /// object.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    pub fn LocalAlloc(flags: UINT, bytes: SIZE_T) -> HLOCAL;
}
