use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    LocalAlloc, VirtualAlloc, VirtualAlloc2, VirtualFree, VirtualFreeEx, ERROR_INVALID_ADDRESS,
    MEM_ADDRESS_REQUIREMENTS, PAGE_NOACCESS,
};
#[allow(unused_imports)]
use std::ptr::null;

/// Allocates memory charges (from the overall size of memory and the paging files on disk) for the
/// specified reserved memory pages. The function also guarantees that when the caller later
/// initially accesses the memory, the contents will be zero. Actual physical pages are not
/// allocated unless/until the virtual addresses are actually accessed.
///
/// To reserve and commit pages in one step, call [`VirtualAlloc`] with `MEM_COMMIT | MEM_RESERVE`.
///
/// Attempting to commit a specific address range by specifying [`MEM_COMMIT`] without
/// [`MEM_RESERVE`] and a non-[`null`] `address` fails unless the entire range has already been
/// reserved. The resulting error code is [`ERROR_INVALID_ADDRESS`].
///
/// An attempt to commit a page that is already committed does not cause the function to fail. This
/// means that you can commit pages without first determining the current commitment state of each
/// page.
///
/// If `address` specifies an address within an enclave, `allocation_type` must be [`MEM_COMMIT`].
pub const MEM_COMMIT: DWORD = 0x00001000;

/// Reserves a range of the process's virtual address space without allocating any actual physical
/// storage in memory or in the paging file on disk.
///
/// You can commit reserved pages in subsequent calls to the [`VirtualAlloc`] function. To reserve
/// and commit pages in one step, call [`VirtualAlloc`] with `MEM_COMMIT | MEM_RESERVE`.
///
/// Other memory allocation functions, such as [`LocalAlloc`], cannot use a reserved range of
/// memory until it is released.
pub const MEM_RESERVE: DWORD = 0x00002000;

/// Replaces a placeholder with a normal private allocation. Only data/pf-backed section views are
/// supported (no images, physical memory, etc.). When you replace a placeholder, `base_address`
/// and `size` must exactly match those of the placeholder, and any provided
/// [`MEM_ADDRESS_REQUIREMENTS`] structure must consist of all zeroes.
///
/// After you replace a placeholder with a private allocation, to free that allocation back to a
/// placeholder, see the `free_type` parameter of [`VirtualFree`] and [`VirtualFreeEx`].
///
/// A placeholder is a type of reserved memory region
pub const MEM_REPLACE_PLACEHOLDER: DWORD = 0x00004000;

/// To create a placeholder, call [`VirtualAlloc2`] with `MEM_RESERVE | MEM_RESERVE_PLACEHOLDER`
/// and `page_protection` set to [`PAGE_NOACCESS`]. To free/split/coalesce a placeholder, see the
/// `free_type` parameter of [`VirtualFree`] and [`VirtualFreeEx`].
///
/// A placeholder is a type of reserved memory region.
pub const MEM_RESERVE_PLACEHOLDER: DWORD = 0x00040000;

/// Indicates that data in the memory range specified by `address` and `size` is no longer of
/// interest. The pages should not be read from or written to the paging file. However, the memory
/// block will be used again later, so it should not be decommitted. This value cannot be used with
/// any other value.
///
/// Using this value does not guarantee that the range operated on with [`MEM_RESET`] will contain
/// zeros. If you want the range to contain zeros, decommit the memory and then recommit it.
///
/// When you specify [`MEM_RESET`], the [`VirtualAlloc`] function ignores the value of `protect`.
/// However, you must still set `protect` to a valid protection value, such as [`PAGE_NOACCESS`].
///
/// [`VirtualAlloc`] returns an error if you use [`MEM_RESET`] and the range of memory is mapped to
/// a file. A shared view is only acceptable if it is mapped to a paging file.
pub const MEM_RESET: DWORD = 0x00080000;

/// Allocates memory at the highest possible address. This can be slower than regular allocations,
/// especially when there are many allocations.
pub const MEM_TOP_DOWN: DWORD = 0x00100000;

/// Causes the system to track pages that are written to in the allocated region. If you specify
/// this value, you must also specify [`MEM_RESERVE`].
///
/// To retrieve the addresses of the pages that have been written to since the region was allocated
/// or the write-tracking state was reset, call the [`GetWriteWatch`] function. To reset the
/// write-tracking state, call [`GetWriteWatch`] or [`ResetWriteWatch`]. The write-tracking feature
/// remains enabled for the memory region until the region is freed.
pub const MEM_WRITE_WATCH: DWORD = 0x00200000;

/// Reserves an address range that can be used to map Address Windowing Extensions (AWE) pages.
///
/// This value must be used with [`MEM_RESERVE`] and no other values.
pub const MEM_PHYSICAL: DWORD = 0x20000000;

/// [`MEM_RESET_UNDO`] should only be called on an address range to which [`MEM_RESET`] was
/// successfully applied earlier. It indicates that the data in the specified memory range
/// specified by `address` and `size` is of interest to the caller and attempts to reverse the
/// effects of [`MEM_RESET`]. If the function succeeds, that means all data in the specified
/// address range is intact. If the function fails, at least some of the data in the address range
/// has been replaced with zeroes.
///
/// This value cannot be used with any other value. If [`MEM_RESET_UNDO`] is called on an address
/// range which was not [`MEM_RESET`] earlier, the behavior is undefined. When you specify
/// [`MEM_RESET`], the [`VirtualAlloc`] function ignores the value of `protect`. However, you must
/// still set `protect` to a valid protection value, such as [`PAGE_NOACCESS`].
///
/// Windows Server 2008 R2, Windows 7, Windows Server 2008, Windows Vista, Windows Server 2003 and
/// Windows XP: The [`MEM_RESET_UNDO`] flag is not supported until Windows 8 and Windows Server
/// 2012.
pub const MEM_RESET_UNDO: DWORD = 0x01000000;

/// Allocates memory using large page support.
///
/// The size and alignment must be a multiple of the large-page minimum. To obtain this value, use
/// the [`GetLargePageMinimum`] function.
///
/// If you specify this value, you must also specify [`MEM_RESERVE`] and [`MEM_COMMIT`].
pub const MEM_LARGE_PAGES: DWORD = 0x20000000;
