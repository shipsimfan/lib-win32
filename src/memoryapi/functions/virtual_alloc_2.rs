use crate::{HANDLE, MEM_EXTENDED_PARAMETER, PVOID, SIZE_T, ULONG};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLargePageMinimum, GetLastError, GetSystemInfo, LocalAlloc, VirtualAlloc, VirtualFree,
    VirtualFreeEx, VirtualProtectEx, ERROR_INVALID_ADDRESS, MEM_ADDRESS_REQUIREMENTS, MEM_COMMIT,
    MEM_EXTENDED_PARAMETER_TYPE, MEM_LARGE_PAGES, MEM_PHYSICAL, MEM_REPLACE_PLACEHOLDER,
    MEM_RESERVE, MEM_RESERVE_PLACEHOLDER, MEM_RESET, MEM_RESET_UNDO, MEM_TOP_DOWN, PAGE_EXECUTE,
    PAGE_EXECUTE_READWRITE, PAGE_GUARD, PAGE_NOACCESS, PAGE_NOCACHE, PAGE_READWRITE,
    PAGE_WRITECOMBINE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Reserves, commits, or changes the state of a region of memory within the virtual address
    /// space of a specified process (allocated memory is initialized to zero).
    ///
    /// # Parameters
    ///  * `process` - The handle to a process. The function allocates memory within the virtual
    ///                address space of this process. The handle must have the
    ///                `PROCESS_VM_OPERATION` access right. If `process` is [`null_mut`], the
    ///                function allocates memory for the calling process.
    ///  * `base_address` - The pointer that specifies a desired starting address for the region of
    ///                     pages that you want to allocate. If `base_address` is [`null_mut`], the
    ///                     function determines where to allocate the region. If `base_address` is
    ///                     not [`null_mut`], then any provided [`MEM_ADDRESS_REQUIREMENTS`]
    ///                     structure must consist of all zeroes, and the base address must be a
    ///                     multiple of the system allocation granularity. To determine the
    ///                     allocation granularity, use the [`GetSystemInfo`] function. If this
    ///                     address is within an enclave that you have not initialized by calling
    ///                     [`InitializeEnclave`], [`VirtualAlloc2`] allocates a page of zeros for
    ///                     the enclave at that address. The page must be previously uncommitted,
    ///                     and will not be measured with the `EEXTEND` instruction of the Intel
    ///                     Software Guard Extensions programming model. If the address in within
    ///                     an enclave that you initialized, then the allocation operation fails
    ///                     with the [`ERROR_INVALID_ADDRESS`] error. That is true for enclaves
    ///                     that do not support dynamic memory management (i.e. SGX1). SGX2
    ///                     enclaves will permit allocation, and the page must be accepted by the
    ///                     enclave after it has been allocated.
    ///  * `size` - The size of the region of memory to allocate, in bytes. The size must always be
    ///             a multiple of the page size. If `base_address` is not [`null_mut`], the
    ///             function allocates all pages that contain one or more bytes in the range from
    ///             `base_address` to `base_address + size`. This means, for example, that a 2-byte
    ///             range that straddles a page boundary causes the function to allocate both
    ///             pages.
    ///  * `allocation_type` - The type of memory allocation. This parameter can contain the
    ///                        following values:
    ///    * [`MEM_COMMIT`] - Allocates memory charges (from the overall size of memory and the
    ///                       paging files on disk) for the specified reserved memory pages. The
    ///                       function also guarantees that when the caller later initially
    ///                       accesses the memory, the contents will be zero. Actual physical pages
    ///                       are not allocated unless/until the virtual addresses are actually
    ///                       accessed. To reserve and commit pages in one step, call
    ///                       [`VirtualAlloc2`] with `MEM_COMMIT | MEM_RESERVE`. Attempting to
    ///                       commit a specific address range by specifying [`MEM_COMMIT`] without
    ///                       [`MEM_RESERVE`] and a non-[`null_mut`] `base_address` fails unless
    ///                       the entire range has already been reserved. The resulting error code
    ///                       is [`ERROR_INVALID_ADDRESS`]. An attempt to commit a page that is
    ///                       already committed does not cause the function to fail. This means
    ///                       that you can commit pages without first determining the current
    ///                       commitment state of each page. If `base_address` specifies an address
    ///                       within an enclave, `allocation_type` must be [`MEM_COMMIT`].
    ///    * [`MEM_RESERVE`] - Reserves a range of the process's virtual address space without
    ///                        allocating any actual physical storage in memory or in the paging
    ///                        file on disk. You commit reserved pages by calling [`VirtualAlloc2`]
    ///                        again with [`MEM_COMMIT`]. To reserve and commit pages in one step,
    ///                        call [`VirtualAlloc2`] with `MEM_COMMIT | MEM_RESERVE`. Other memory
    ///                        allocation functions, such as [`LocalAlloc`], cannot use reserved
    ///                        memory until it has been released.
    ///    * [`MEM_REPLACE_PLACEHOLDER`] - Replaces a placeholder with a normal private allocation.
    ///                                    Only data/pf-backed section views are supported (no
    ///                                    images, physical memory, etc.). When you replace a
    ///                                    placeholder, `base_address` and `size` must exactly
    ///                                    match those of the placeholder, and any provided
    ///                                    [`MEM_ADDRESS_REQUIREMENTS`] structure must consist of
    ///                                    all zeroes. After you replace a placeholder with a
    ///                                    private allocation, to free that allocation back to a
    ///                                    placeholder, see the `free_type` parameter of
    ///                                    [`VirtualFree`] and [`VirtualFreeEx`]. A placeholder is
    ///                                    a type of reserved memory region.
    ///    * [`MEM_RESERVE_PLACEHOLDER`] - To create a placeholder, call [`VirtualAlloc2`] with
    ///                                    `MEM_RESERVE | MEM_RESERVE_PLACEHOLDER` and
    ///                                    `page_protection` set to [`PAGE_NOACCESS`]. To
    ///                                    free/split/coalesce a placeholder, see the `free_type`
    ///                                    parameter of [`VirtualFree`] and [`VirtualFreeEx`]. A
    ///                                    placeholder is a type of reserved memory region.
    ///    * [`MEM_RESET`] - Indicates that data in the memory range specified by `base_address`
    ///                      and `size` is no longer of interest. The pages should not be read from
    ///                      or written to the paging file. However, the memory block will be used
    ///                      again later, so it should not be decommitted. This value cannot be
    ///                      used with any other value. Using this value does not guarantee that
    ///                      the range operated on with [`MEM_RESET`] will contain zeros. If you
    ///                      want the range to contain zeros, decommit the memory and then recommit
    ///                      it. When you use [`MEM_RESET`], the [`VirtualAlloc2`] function ignores
    ///                      the value of `protect`. However, you must still set `protect` to a
    ///                      valid protection value, such as [`PAGE_NOACCESS`]. [`VirtualAlloc2`]
    ///                      returns an error if you use [`MEM_RESET`] and the range of memory is
    ///                      mapped to a file. A shared view is only acceptable if it is mapped to
    ///                      a paging file.
    ///    * [`MEM_RESET_UNDO`] - [`MEM_RESET_UNDO`] should only be called on an address range to
    ///                           which [`MEM_RESET`] was successfully applied earlier. It
    ///                           indicates that the data in the specified memory range specified
    ///                           by `base_address` and `size` is of interest to the caller and
    ///                           attempts to reverse the effects of [`MEM_RESET`]. If the function
    ///                           succeeds, that means all data in the specified address range is
    ///                           intact. If the function fails, at least some of the data in the
    ///                           address range has been replaced with zeroes. This value cannot be
    ///                           used with any other value. If [`MEM_RESET_UNDO`] is called on an
    ///                           address range which was not [`MEM_RESET`] earlier, the behavior
    ///                           is undefined. When you specify [`MEM_RESET`], the
    ///                           [`VirtualAlloc2`] function ignores the value of
    ///                           `page_protection`. However, you must still set `page_protection`
    ///                           to a valid protection value, such as [`PAGE_NOACCESS`]. Windows
    ///                           Server 2008 R2, Windows 7, Windows Server 2008, Windows Vista,
    ///                           Windows Server 2003 and Windows XP:  The [`MEM_RESET_UNDO`] flag
    ///                           is not supported until Windows 8 and Windows Server 2012.
    ///    * [`MEM_LARGE_PAGES`] - Allocates memory using large page support. The size and
    ///                            alignment must be a multiple of the large-page minimum. To
    ///                            obtain this value, use the [`GetLargePageMinimum`] function. If
    ///                            you specify this value, you must also specify [`MEM_RESERVE`]
    ///                            and [`MEM_COMMIT`].
    ///    * [`MEM_PHYSICAL`] - Reserves an address range that can be used to map Address Windowing
    ///                         Extensions (AWE) pages. This value must be used with
    ///                         [`MEM_RESERVE`] and no other values.
    ///    * [`MEM_TOP_DOWN`] - Allocates memory at the highest possible address. This can be
    ///                         slower than regular allocations, especially when there are many
    ///                         allocations.
    ///  * `page_protection` - The memory protection for the region of pages to be allocated. If
    ///                        the pages are being committed, you can specify any one of the memory
    ///                        protection constants. When allocating dynamic memory for an enclave,
    ///                        the `page_protection` parameter must be [`PAGE_READWRITE`] or
    ///                        [`PAGE_EXECUTE_READWRITE`]. If `base_address` specifies an address
    ///                        within an enclave, `page_protection` cannot be any of the following
    ///                        values:
    ///    * [`PAGE_NOACCESS`]
    ///    * [`PAGE_GUARD`]
    ///    * [`PAGE_NOCACHE`]
    ///    * [`PAGE_WRITECOMBINE`]
    ///  * `extended_parameters` - An optional pointer to one or more extended parameters of type
    ///                            [`MEM_EXTENDED_PARAMETER`]. Each of those extended parameter
    ///                            values can itself have a `r#type` field of either
    ///                            [`MEM_EXTENDED_PARAMETER_TYPE::MemExtendedParameterAddressRequirements`]
    ///                            or [`MEM_EXTENDED_PARAMETER_TYPE::MemExtendedParameterNumaNode`].
    ///                            If no [`MEM_EXTENDED_PARAMETER_TYPE::MemExtendedParameterNumaNode`]
    ///                            extended parameter is provided, then the behavior is the same as
    ///                            for the [`VirtualAlloc`]/[`MapViewOfFile`] functions (that is,
    ///                            the preferred NUMA node for the physical pages is determined
    ///                            based on the ideal processor of the thread that first accesses
    ///                            the memory).
    ///  * parameter_count - The number of extended parameters pointed to by `extended_parameters`.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the base address of the allocated region of
    /// pages.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// This function lets you specify:
    ///  - a range of virtual address space and a power-of-2 alignment restriction for new
    ///    allocations
    ///  - an arbitrary number of extended parameters
    ///  - a preferred NUMA node for the physical memory as an extended parameter (see the
    ///    `extended_parameters` parameter)
    ///  - a placeholder operation (specifically, replacement).
    ///
    /// This API provides specialized techniques for managing virtual memory in support of
    /// high-performance games and server applications. For example, placeholders allow a reserved
    /// memory range to be explicitly partitioned, overlaid, and re-mapped; this can be used to
    /// implement arbitrarily extendable regions or virtual memory ring buffers. [`VirtualAlloc2`]
    /// also allows for allocating memory with a specific memory-alignment.
    ///
    /// Each page has an associated page state. The [`VirtualAlloc2`] function can perform the
    /// following operations:
    ///  - Commit a region of reserved pages
    ///  - Reserve a region of free pages
    ///  - Simultaneously reserve and commit a region of free pages
    ///
    /// [`VirtualAlloc2`] can commit pages that are already committed, but cannot reserve pages
    /// that are already reserved. This means you can commit a range of pages, regardless of
    /// whether they have already been committed, and the function will not fail. In general
    /// however, only a minimal range of mostly uncommitted pages should be specified, because
    /// committing a large number of pages that are already committed can cause the
    /// [`VirtualAlloc2`] call to take much longer.
    ///
    /// You can use [`VirtualAlloc2`] to reserve a block of pages and then make additional calls to
    /// [`VirtualAlloc2`] to commit individual pages from the reserved block. This enables a
    /// process to reserve a range of its virtual address space without consuming physical storage
    /// until it is needed.
    ///
    /// If the `address` parameter is not [`null_mut`], the function uses the `address` and `size`
    /// parameters to compute the region of pages to be allocated. The current state of the entire
    /// range of pages must be compatible with the type of allocation specified by the
    /// `allocation_type` parameter. Otherwise, the function fails and none of the pages is
    /// allocated. This compatibility requirement does not preclude committing an already committed
    /// page; see the preceding list.
    ///
    /// To execute dynamically generated code, use [`VirtualAlloc2`] to allocate memory, and the
    /// [`VirtualProtectEx`] function to grant [`PAGE_EXECUTE`] access.
    ///
    /// The [`VirtualAlloc2`] function can be used to reserve an Address Windowing Extensions (AWE)
    /// region of memory within the virtual address space of a specified process. This region of
    /// memory can then be used to map physical pages into and out of virtual memory as required by
    /// the application. The [`MEM_PHYSICAL`] and [`MEM_RESERVE`] values must be set in the
    /// `allocation_type` parameter. The [`MEM_COMMIT`] value must not be set. The page protection
    /// must be set to [`PAGE_READWRITE`].
    ///
    /// The [`VirtualFreeEx`] function can decommit a committed page, releasing the page's storage,
    /// or it can simultaneously decommit and release a committed page. It can also release a
    /// reserved page, making it a free page.
    ///
    /// When creating a region that will be executable, the calling program bears responsibility
    /// for ensuring cache coherency via an appropriate call to [`FlushInstructionCache`] once the
    /// code has been set in place. Otherwise attempts to execute code out of the newly executable
    /// region may produce unpredictable results.
    pub fn VirtualAlloc2(
        process: HANDLE,
        base_address: PVOID,
        size: SIZE_T,
        allocation_type: ULONG,
        page_protection: ULONG,
        extended_parameters: *mut MEM_EXTENDED_PARAMETER,
        parameter_count: ULONG,
    ) -> PVOID;
}
