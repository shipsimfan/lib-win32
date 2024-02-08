use crate::{HGLOBAL, SIZE_T, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, GlobalFree, GlobalLock, GlobalSize, LocalAlloc, LocalFree, GHND, GMEM_FIXED,
    GMEM_MOVEABLE, GMEM_ZEROINIT, GPTR, PAGE_EXECUTE, VirtualAlloc, VirtualProtect
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Allocates the specified number of bytes from the heap.
    ///
    /// # Parameters
    ///  * `flags` - The memory allocation attributes. If zero is specified, the default is
    ///              [`GMEM_FIXED`]. This parameter can be one or more of the following values,
    ///              except for the incompatible combinations that are specifically noted:
    ///    * [`GHND`] - Combines [`GMEM_MOVEABLE`] and [`GMEM_ZEROINIT`].
    ///    * [`GMEM_FIXED`] - Allocates fixed memory. The return value is a pointer.
    ///    * [`GMEM_MOVEABLE`] - Allocates movable memory. Memory blocks are never moved in
    ///                          physical memory, but they can be moved within the default heap.
    ///                          The return value is a handle to the memory object. To translate
    ///                          the handle into a pointer, use the [`GlobalLock`] function. This
    ///                          value cannot be combined with [`GMEM_FIXED`].
    ///    * [`GMEM_ZEROINIT`] - Initializes memory contents to zero.
    ///    * [`GPTR`] - Combines [`GMEM_FIXED`] and [`GMEM_ZEROINIT`].
    ///  * `bytes` - The number of bytes to allocate. If this parameter is zero and the `flags`
    ///              parameter specifies [`GMEM_MOVEABLE`], the function returns a handle to a
    ///              memory object that is marked as discarded.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the newly allocated memory
    /// object.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// Windows memory management does not provide a separate local heap and global heap.
    /// Therefore, the [`GlobalAlloc`] and [`LocalAlloc`] functions are essentially the same.
    ///
    /// The movable-memory flags [`GHND`] and [`GMEM_MOVEABLE`] add unnecessary overhead and require
    /// locking to be used safely. They should be avoided unless documentation specifically states
    /// that they should be used.
    ///
    /// New applications should use the heap functions to allocate and manage memory unless the
    /// documentation specifically states that a global function should be used. For example, the
    /// global functions are still used with Dynamic Data Exchange (DDE), the clipboard functions,
    /// and OLE data objects.
    ///
    /// If the [`GlobalAlloc`] function succeeds, it allocates at least the amount of memory
    /// requested. If the actual amount allocated is greater than the amount requested, the process
    /// can use the entire amount. To determine the actual number of bytes allocated, use the
    /// [`GlobalSize`] function.
    ///
    /// If the heap does not contain sufficient free space to satisfy the request, [`GlobalAlloc`]
    /// returns [`null_mut`]. Because [`null_mut`] is used to indicate an error, virtual address
    /// zero is never allocated. It is, therefore, easy to detect the use of a [`null_mut`]
    /// pointer.
    ///
    /// Memory allocated with this function is guaranteed to be aligned on an 8-byte boundary. To
    /// execute dynamically generated code, use the [`VirtualAlloc`] function to allocate memory
    /// and the [`VirtualProtect`] function to grant [`PAGE_EXECUTE`] access.
    ///
    /// To free the memory, use the [`GlobalFree`] function. It is not safe to free memory
    /// allocated with [`GlobalAlloc`] using [`LocalFree`].
    pub fn GlobalAlloc(flags: UINT, bytes: SIZE_T) -> HGLOBAL;
}
