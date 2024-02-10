use crate::{BOOL, DWORD, HANDLE, LPVOID, SIZE_T};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, VirtualAlloc, VirtualAllocEx, VirtualFree, MEM_COALESCE_PLACEHOLDERS,
    MEM_DECOMMIT, MEM_PRESERVE_PLACEHOLDER, MEM_RELEASE,
};

#[link(name = "Kernel32")]
extern "system" {
    /// Releases, decommits, or releases and decommits a region of memory within the virtual
    /// address space of a specified process.
    ///
    /// # Parameters
    ///  * `process` - A handle to a process. The function frees memory within the virtual address
    ///                space of the process. The handle must have the `PROCESS_VM_OPERATION` access
    ///                right.
    ///  * `address` - A pointer to the starting address of the region of memory to be freed. If
    ///                the `free_type` parameter is [`MEM_RELEASE`], `address` must be the base
    ///                address returned by the [`VirtualAllocEx`] function when the region is
    ///                reserved.
    ///  * `size` - The size of the region of memory to free, in bytes. If the `free_type`
    ///             parameter is [`MEM_RELEASE`], `size` must be 0 (zero). The function frees the
    ///             entire region that is reserved in the initial allocation call to
    ///             [`VirtualAllocEx`]. If `free_type` is [`MEM_DECOMMIT`], the function decommits
    ///             all memory pages that contain one or more bytes in the range from the `address`
    ///             parameter to `address + size`. This means, for example, that a 2-byte region of
    ///             memory that straddles a page boundary causes both pages to be decommitted. If
    ///             `address` is the base address returned by [`VirtualAllocEx`] and `size` is 0
    ///             (zero), the function decommits the entire region that is allocated by
    ///             [`VirtualAllocEx`]. After that, the entire region is in the reserved state.
    ///  * `free_type` - The type of free operation. This parameter can contain the following
    ///                  values:
    ///   * [`MEM_DECOMMIT`] - Decommits the specified region of committed pages. After the
    ///                        operation, the pages are in the reserved state. The function does
    ///                        not fail if you attempt to decommit an uncommitted page. This means
    ///                        that you can decommit a range of pages without first determining the
    ///                        current commitment state. The [`MEM_DECOMMIT`] value is not
    ///                        supported when the `address` parameter provides the base address for
    ///                        an enclave. This is true for enclaves that do not support dynamic
    ///                        memory management (i.e. SGX1). SGX2 enclaves permit [`MEM_DECOMMIT`]
    ///                        anywhere in the enclave.
    ///    * [`MEM_RELEASE`] - Releases the specified region of pages, or placeholder (for a
    ///                        placeholder, the address space is released and available for other
    ///                        allocations). After this operation, the pages are in the free state.
    ///                        If you specify this value, `size` must be 0 (zero), and `address`
    ///                        must point to the base address returned by the [`VirtualAlloc`]
    ///                        function when the region is reserved. The function fails if either
    ///                        of these conditions is not met. If any pages in the region are
    ///                        committed currently, the function first decommits, and then releases
    ///                        them. The function does not fail if you attempt to release pages
    ///                        that are in different states, some reserved and some committed. This
    ///                        means that you can release a range of pages without first
    ///                        determining the current commitment state.
    ///    * [`MEM_COALESCE_PLACEHOLDERS`] - To coalesce two adjacent placeholders, specify
    ///                                      `MEM_RELEASE | MEM_COALESCE_PLACEHOLDERS`. When you
    ///                                      coalesce placeholders, `address` and `size` must
    ///                                      exactly match the overall range of the placeholders to
    ///                                      be merged.
    ///    * [`MEM_PRESERVE_PLACEHOLDER`] - Frees an allocation back to a placeholder (after you've
    ///                                     replaced a placeholder with a private allocation using
    ///                                     [`VirtualAlloc2`] or [`Virtual2AllocFromApp`]). To
    ///                                     split a placeholder into two placeholders, specify
    ///                                     `MEM_RELEASE | MEM_PRESERVE_PLACEHOLDER`.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is 0 (zero). To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// Each page of memory in a process virtual address space has a Page State. The
    /// [`VirtualFree`] function can decommit a range of pages that are in different states, some
    /// committed and some uncommitted. This means that you can decommit a range of pages without
    /// first determining the current commitment state of each page. Decommitting a page releases
    /// its physical storage, either in memory or in the paging file on disk.
    ///
    /// If a page is decommitted but not released, its state changes to reserved. Subsequently, you
    /// can call [`VirtualAlloc`] to commit it, or [`VirtualFree`] to release it. Attempts to read
    /// from or write to a reserved page results in an access violation exception.
    ///
    /// The [`VirtualFree`] function can release a range of pages that are in different states,
    /// some reserved and some committed. This means that you can release a range of pages without
    /// first determining the current commitment state of each page. The entire range of pages
    /// originally reserved by the [`VirtualAlloc`] function must be released at the same time.
    ///
    /// If a page is released, its state changes to free, and it is available for subsequent
    /// allocation operations. After memory is released or decommited, you can never refer to the
    /// memory again. Any information that may have been in that memory is gone forever. Attempting
    /// to read from or write to a free page results in an access violation exception. If you need
    /// to keep information, do not decommit or free memory that contains the information.
    ///
    /// The [`VirtualFree`] function can be used on an AWE region of memory, and it invalidates
    /// any physical page mappings in the region when freeing the address space. However, the
    /// physical page is not deleted, and the application can use them. The application must
    /// explicitly call [`FreeUserPhysicalPages`] to free the physical pages. When the process is
    /// terminated, all resources are cleaned up automatically.
    ///
    /// Windows 10, version 1709 and later and Windows 11: To delete the enclave when you finish
    /// using it, call [`DeleteEnclave`]. You cannot delete a VBS enclave by calling the
    /// [`VirtualFree`] or [`VirtualFreeEx`] function. You can still delete an SGX enclave by
    /// calling [`VirtualFree`] or [`VirtualFreeEx`].
    ///
    /// Windows 10, version 1507, Windows 10, version 1511, Windows 10, version 1607 and Windows
    /// 10, version 1703: To delete the enclave when you finish using it, call the [`VirtualFree`]
    /// or [`VirtualFreeEx`] function and specify the following values:
    ///  - The base address of the enclave for the `address` parameter.
    ///  - 0 for the `size` parameter.
    ///  - [`MEM_RELEASE`] for the `free_type` parameter.
    pub fn VirtualFreeEx(process: HANDLE, address: LPVOID, size: SIZE_T, free_type: DWORD) -> BOOL;
}
