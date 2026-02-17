use crate::{HANDLE, MEM_EXTENDED_PARAMETER, PVOID, SIZE_T, ULONG, ULONG64};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLargePageMinimum, GetLastError, VirtualAlloc, MEM_ADDRESS_REQUIREMENTS,
    MEM_EXTENDED_PARAMETER_TYPE, MEM_LARGE_PAGES, MEM_REPLACE_PLACEHOLDER, MEM_RESERVE,
    PAGE_READONLY, SEC_IMAGE, SEC_LARGE_PAGES,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Maps a view of a file or a pagefile-backed section into the address space of the specified
    /// process.
    ///
    /// Using this function, you can: for new allocations, specify a range of virtual address space
    /// and a power-of-2 alignment restriction; specify an arbitrary number of extended parameters;
    /// specify a preferred NUMA node for the physical memory as an extended parameter; and specify
    /// a placeholder operation (specifically, replacement).
    ///
    /// To specify the NUMA node, see the `extended_parameters` parameter.
    ///
    /// # Parameters
    ///  * `file_mapping` - A [`HANDLE`] to a section that is to be mapped into the address space
    ///                     of the specified process.
    ///  * `process` - A [`HANDLE`] to a process into which the section will be mapped.
    ///  * `base_address` - The desired base address of the view. The address is rounded down to
    ///                     the nearest 64k boundary. If this parameter is [`null_mut`], the system
    ///                     picks the base address. If `base_address` is not [`null_mut`], then any
    ///                     provided [`MEM_ADDRESS_REQUIREMENTS`] must consist of all zeroes.
    ///  * `offset` - The offset from the beginning of the section. This must be 64k aligned.
    ///  * `view_size` - The number of bytes to map. A value of zero (0) specifies that the entire
    ///                  section is to be mapped. The size must always be a multiple of the page
    ///                  size.
    ///  * `allocation_type` - The type of memory allocation. This parameter can be zero (0) or one
    ///                        of the following values:
    ///    * [`MEM_RESERVE`] - Maps a reserved view.
    ///    * [`MEM_REPLACE_PLACEHOLDER`] - Replaces a placeholder with a mapped view. Only
    ///                                    data/pf-backed section views are supported (no images,
    ///                                    physical memory, etc.). When you replace a placeholder,
    ///                                    `base_address` and `view_size` must exactly match those
    ///                                    of the placeholder, and any provided
    ///                                    [`MEM_ADDRESS_REQUIREMENTS`] structure must consist of
    ///                                    all zeroes. After you replace a placeholder with a
    ///                                    mapped view, to free that mapped view back to a
    ///                                    placeholder, see the `unmap_flags` parameter of
    ///                                    [`UnmapViewOfFileEx`] and [`UnmapViewOfFile2`]. A
    ///                                    placeholder is a type of reserved memory region. The 64k
    ///                                    alignment requirements on `offset` and `base_address` do
    ///                                    not apply when this flag is specified.
    ///    * [`MEM_LARGE_PAGES`] - Maps a large page view. This flag specifies that the view should
    ///                            be mapped using large page support. The size of the view must be
    ///                            a multiple of the size of a large page reported by the
    ///                            [`GetLargePageMinimum`] function, and the file-mapping object
    ///                            must have been created using the [`SEC_LARGE_PAGES`] option. If
    ///                            you provide a non-[`null_mut`] value for the `base_address`
    ///                            parameter, then the value must be a multiple of
    ///                            [`GetLargePageMinimum`].
    ///  * `page_protection` - The desired page protection. For file-mapping objects created with
    ///                        the [`SEC_IMAGE`] attribute, the `page_protection` parameter has no
    ///                        effect, and should be set to any valid value such as
    ///                        [`PAGE_READONLY`].
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
    ///  * `parameter_count` - The number of extended parameters pointed to by
    ///                        `extended_parameters`.
    ///
    /// # Return Value
    /// Returns the base address of the mapped view, if successful. Otherwise, returns [`null_mut`]
    /// and extended error status is available using [`GetLastError`].
    ///
    /// # Remarks
    /// This API helps support high-performance games, and server applications, which have
    /// particular requirements around managing their virtual address space. For example, mapping
    /// memory on top of a previously reserved region; this is useful for implementing an
    /// automatically wrapping ring buffer. And allocating memory with specific alignment; for
    /// example, to enable your application to commit large/huge page-mapped regions on demand.
    pub fn MapViewOfFile3(
        file_mapping: HANDLE,
        process: HANDLE,
        base_address: PVOID,
        offset: ULONG64,
        view_size: SIZE_T,
        allocation_type: ULONG,
        page_protection: ULONG,
        extended_parameters: *mut MEM_EXTENDED_PARAMETER,
        parameter_count: ULONG,
    ) -> PVOID;
}
