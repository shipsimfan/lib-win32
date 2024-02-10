use crate::{DWORD, LPVOID, SIZE_T};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, LocalAlloc, VirtualAllocEx, VirtualFree, VirtualProtect, ERROR_INVALID_ADDRESS,
    MEM_COMMIT, MEM_LARGE_PAGES, MEM_PHYSICAL, MEM_RESERVE, MEM_RESET, MEM_RESET_UNDO,
    MEM_TOP_DOWN, MEM_WRITE_WATCH, PAGE_EXECUTE, PAGE_EXECUTE_READWRITE, PAGE_GUARD, PAGE_NOACCESS,
    PAGE_NOCACHE, PAGE_READWRITE, PAGE_WRITECOMBINE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Reserves, commits, or changes the state of a region of pages in the virtual address space
    /// of the calling process. Memory allocated by this function is automatically initialized to
    /// zero.
    ///
    /// To allocate memory in the address space of another process, use the [`VirtualAllocEx`]
    /// function.
    ///
    /// # Parameters
    ///  * `address` - The starting address of the region to allocate. If the memory is being
    ///                reserved, the specified address is rounded down to the nearest multiple of
    ///                the allocation granularity. If the memory is already reserved and is being
    ///                committed, the address is rounded down to the next page boundary. To
    ///                determine the size of a page and the allocation granularity on the host
    ///                computer, use the [`GetSystemInfo`] function. If this parameter is
    ///                [`null_mut`], the system determines where to allocate the region. If this
    ///                address is within an enclave that you have not initialized by calling
    ///                [`InitializeEnclave`], [`VirtualAlloc`] allocates a page of zeros for the
    ///                enclave at that address. The page must be previously uncommitted, and will
    ///                not be measured with the `EEXTEND` instruction of the Intel Software Guard
    ///                Extensions programming model. If the address is within an enclave that you
    ///                initialized, then the allocation operation fails with the
    ///                [`ERROR_INVALID_ADDRESS`] error. That is true for enclaves that do not
    ///                support dynamic memory management (i.e. SGX1). SGX2 enclaves will permit
    ///                allocation, and the page must be accepted by the enclave after it has been
    ///                allocated.
    ///  * `size` - The size of the region, in bytes. If the `address` parameter is [`null_mut`],
    ///             this value is rounded up to the next page boundary. Otherwise, the allocated
    ///             pages include all pages containing one or more bytes in the range from
    ///             `address` to `address + size`. This means that a 2-byte range straddling a page
    ///             boundary causes both pages to be included in the allocated region.
    ///  * `allocation_type` - The type of memory allocation. This parameter can the following
    ///                        values:
    ///    * [`MEM_COMMIT`] - Allocates memory charges (from the overall size of memory and the
    ///                       paging files on disk) for the specified reserved memory pages. The
    ///                       function also guarantees that when the caller later initially
    ///                       accesses the memory, the contents will be zero. Actual physical pages
    ///                       are not allocated unless/until the virtual addresses are actually
    ///                       accessed. To reserve and commit pages in one step, call
    ///                       [`VirtualAlloc`] with `MEM_COMMIT | MEM_RESERVE`. Attempting to
    ///                       commit a specific address range by specifying [`MEM_COMMIT`] without
    ///                       [`MEM_RESERVE`] and a non-[`null_mut`] `address` fails unless the
    ///                       entire range has already been reserved. The resulting error code is
    ///                       [`ERROR_INVALID_ADDRESS`]. An attempt to commit a page that is
    ///                       already committed does not cause the function to fail. This means
    ///                       that you can commit pages without first determining the current
    ///                       commitment state of each page. If `address` specifies an address
    ///                       within an enclave, `allocation_type` must be [`MEM_COMMIT`].
    ///    * [`MEM_RESERVE`] - Reserves a range of the process's virtual address space without
    ///                        allocating any actual physical storage in memory or in the paging
    ///                        file on disk. You can commit reserved pages in subsequent calls to
    ///                        the [`VirtualAlloc`] function. To reserve and commit pages in one
    ///                        step, call [`VirtualAlloc`] with `MEM_COMMIT | MEM_RESERVE`. Other
    ///                        memory allocation functions, such as [`LocalAlloc`], cannot use a
    ///                        reserved range of memory until it is released.
    ///    * [`MEM_RESET`] - Indicates that data in the memory range specified by `address` and
    ///                      `size` is no longer of interest. The pages should not be read from or
    ///                      written to the paging file. However, the memory block will be used
    ///                      again later, so it should not be decommitted. This value cannot be
    ///                      used with any other value. Using this value does not guarantee that
    ///                      the range operated on with [`MEM_RESET`] will contain zeros. If you
    ///                      want the range to contain zeros, decommit the memory and then recommit
    ///                      it. When you specify [`MEM_RESET`], the [`VirtualAlloc`] function
    ///                      ignores the value of `protect`. However, you must still set `protect`
    ///                      to a valid protection value, such as [`PAGE_NOACCESS`].
    ///                      [`VirtualAlloc`] returns an error if you use [`MEM_RESET`] and the
    ///                      range of memory is mapped to a file. A shared view is only acceptable
    ///                      if it is mapped to a paging file.
    ///    * [`MEM_RESET_UNDO`] - [`MEM_RESET_UNDO`] should only be called on an address range to
    ///                           which [`MEM_RESET`] was successfully applied earlier. It
    ///                           indicates that the data in the specified memory range specified
    ///                           by `address` and `size` is of interest to the caller and attempts
    ///                           to reverse the effects of [`MEM_RESET`]. If the function
    ///                           succeeds, that means all data in the specified address range is
    ///                           intact. If the function fails, at least some of the data in the
    ///                           address range has been replaced with zeroes. This value cannot be
    ///                           used with any other value. If [`MEM_RESET_UNDO`] is called on an
    ///                           address range which was not [`MEM_RESET`] earlier, the behavior
    ///                           is undefined. When you specify [`MEM_RESET`], the
    ///                           [`VirtualAlloc`] function ignores the value of `protect`.
    ///                           However, you must still set `protect` to a valid protection
    ///                           value, such as [`PAGE_NOACCESS`]. Windows Server 2008 R2, Windows
    ///                           7, Windows Server 2008, Windows Vista, Windows Server 2003 and
    ///                           Windows XP: The [`MEM_RESET_UNDO`] flag is not supported until
    ///                           Windows 8 and Windows Server 2012.
    ///    * [`MEM_LARGE_PAGES`] - 	Allocates memory using large page support. The size and
    ///                             alignment must be a multiple of the large-page minimum. To
    ///                             obtain this value, use the [`GetLargePageMinimum`] function.
    ///                             If you specify this value, you must also specify
    ///                             [`MEM_RESERVE`] and [`MEM_COMMIT`].
    ///    * [`MEM_PHYSICAL`] - Reserves an address range that can be used to map Address Windowing
    ///                         Extensions (AWE) pages. This value must be used with
    ///                         [`MEM_RESERVE`] and no other values.
    ///    * [`MEM_TOP_DOWN`] - Allocates memory at the highest possible address. This can be
    ///                         slower than regular allocations, especially when there are many
    ///                         allocations.
    ///    * [`MEM_WRITE_WATCH`] - Causes the system to track pages that are written to in the
    ///                            allocated region. If you specify this value, you must also
    ///                            specify [`MEM_RESERVE`]. To retrieve the addresses of the pages
    ///                            that have been written to since the region was allocated or the
    ///                            write-tracking state was reset, call the [`GetWriteWatch`]
    ///                            function. To reset the write-tracking state, call
    ///                            [`GetWriteWatch`] or [`ResetWriteWatch`]. The write-tracking
    ///                            feature remains enabled for the memory region until the region
    ///                            is freed.
    ///  * `protect` - The memory protection for the region of pages to be allocated. If the pages
    ///                are being committed, you can specify any one of the memory protection
    ///                constants. When allocating dynamic memory for an enclave, the `protect`
    ///                parameter must be [`PAGE_READWRITE`] or [`PAGE_EXECUTE_READWRITE`]. If
    ///                `address` specifies an address within an enclave, `protect` cannot be any of
    ///                the following values:
    ///    * [`PAGE_NOACCESS`]
    ///    * [`PAGE_GUARD`]
    ///    * [`PAGE_NOCACHE`]
    ///    * [`PAGE_WRITECOMBINE`]
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the base address of the allocated region of
    /// pages.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// Each page has an associated page state. The [`VirtualAlloc`] function can perform the
    /// following operations:
    ///  - Commit a region of reserved pages
    ///  - Reserve a region of free pages
    ///  - Simultaneously reserve and commit a region of free pages
    ///
    /// [`VirtualAlloc`] cannot reserve a reserved page. It can commit a page that is already
    /// committed. This means you can commit a range of pages, regardless of whether they have
    /// already been committed, and the function will not fail.
    ///
    /// You can use [`VirtualAlloc`] to reserve a block of pages and then make additional calls to
    /// [`VirtualAlloc`] to commit individual pages from the reserved block. This enables a process
    /// to reserve a range of its virtual address space without consuming physical storage until it
    /// is needed.
    ///
    /// If the `address` parameter is not [`null_mut`], the function uses the `address` and `size`
    /// parameters to compute the region of pages to be allocated. The current state of the entire
    /// range of pages must be compatible with the type of allocation specified by the
    /// `allocation_type` parameter. Otherwise, the function fails and none of the pages are
    /// allocated. This compatibility requirement does not preclude committing an already committed
    /// page, as mentioned previously.
    ///
    /// To execute dynamically generated code, use [`VirtualAlloc`] to allocate memory and the
    /// [`VirtualProtect`] function to grant [`PAGE_EXECUTE`] access.
    ///
    /// The [`VirtualAlloc`] function can be used to reserve an Address Windowing Extensions (AWE)
    /// region of memory within the virtual address space of a specified process. This region of
    /// memory can then be used to map physical pages into and out of virtual memory as required by
    /// the application. The [`MEM_PHYSICAL`] and [`MEM_RESERVE`] values must be set in the
    /// `allocation_type` parameter. The [`MEM_COMMIT`] value must not be set. The page protection
    /// must be set to [`PAGE_READWRITE`].
    ///
    /// The [`VirtualFree`] function can decommit a committed page, releasing the page's storage,
    /// or it can simultaneously decommit and release a committed page. It can also release a
    /// reserved page, making it a free page.
    ///
    /// When creating a region that will be executable, the calling program bears responsibility
    /// for ensuring cache coherency via an appropriate call to [`FlushInstructionCache`] once the
    /// code has been set in place. Otherwise attempts to execute code out of the newly executable
    /// region may produce unpredictable results.
    pub fn VirtualAlloc(
        address: LPVOID,
        size: SIZE_T,
        allocation_type: DWORD,
        protect: DWORD,
    ) -> LPVOID;
}
