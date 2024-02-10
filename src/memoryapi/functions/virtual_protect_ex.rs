use crate::{BOOL, DWORD, HANDLE, LPVOID, PDWORD, SIZE_T};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, GlobalAlloc, LocalAlloc, VirtualAlloc, VirtualAllocEx, MEM_RESERVE, PAGE_GUARD,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Changes the protection on a region of committed pages in the virtual address space of a
    /// specified process.
    ///
    /// # Parameters
    ///  * `process` - A handle to the process whose memory protection is to be changed. The handle
    ///                must have the `PROCESS_VM_OPERATION` access right.
    ///  * `address` - A pointer to the base address of the region of pages whose access protection
    ///                attributes are to be changed. All pages in the specified region must be
    ///                within the same reserved region allocated when calling the [`VirtualAlloc`]
    ///                or [`VirtualAllocEx`] function using [`MEM_RESERVE`]. The pages cannot span
    ///                adjacent reserved regions that were allocated by separate calls to
    ///                [`VirtualAlloc`] or [`VirtualAllocEx`] using [`MEM_RESERVE`].
    ///  * `size` - The size of the region whose access protection attributes are changed, in
    ///             bytes. The region of affected pages includes all pages containing one or more
    ///             bytes in the range from the `address` parameter to `address + size`. This means
    ///             that a 2-byte range straddling a page boundary causes the protection attributes
    ///             of both pages to be changed.
    ///  * `new_protect` - The memory protection option. This parameter can be one of the memory
    ///                    protection constants. For mapped views, this value must be compatible
    ///                    with the access protection specified when the view was mapped (see
    ///                    [`MapViewOfFile`], [`MapViewOfFileEx`], and [`MapViewOfFileExNuma`]).
    ///  * `old_protect` - A pointer to a variable that receives the previous access protection of
    ///                    the first page in the specified region of pages. If this parameter is
    ///                    [`null_mut`] or does not point to a valid variable, the function fails.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// The access protection value can be set only on committed pages. If the state of any page in
    /// the specified region is not committed, the function fails and returns without modifying the
    /// access protection of any pages in the specified region.
    ///
    /// The [`PAGE_GUARD`] protection modifier establishes guard pages. Guard pages act as one-shot
    /// access alarms.
    ///
    /// It is best to avoid using [`VirtualProtectEx`] to change page protections on memory blocks
    /// allocated by [`GlobalAlloc`], [`HeapAlloc`], or [`LocalAlloc`], because multiple memory
    /// blocks can exist on a single page. The heap manager assumes that all pages in the heap
    /// grant at least read and write access.
    ///
    /// When protecting a region that will be executable, the calling program bears responsibility
    /// for ensuring cache coherency via an appropriate call to [`FlushInstructionCache`] once the
    /// code has been set in place. Otherwise attempts to execute code out of the newly executable
    /// region may produce unpredictable results.
    pub fn VirtualProtectEx(
        process: HANDLE,
        address: LPVOID,
        size: SIZE_T,
        new_protect: DWORD,
        old_protect: PDWORD,
    ) -> BOOL;
}
