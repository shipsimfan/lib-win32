use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{VirtualAllocExNuma, INVALID_HANDLE_VALUE, PAGE_READONLY};

/// Sets the file that is specified to be an executable image file.
///
/// The [`SEC_IMAGE`] attribute must be combined with a page protection value such as
/// [`PAGE_READONLY`]. However, this page protection value has no effect on views of the executable
/// image file. Page protection for views of an executable image file is determined by the
/// executable file itself.
///
/// No other attributes are valid with [`SEC_IMAGE`].
pub const SEC_IMAGE: DWORD = 0x01000000;

/// Reserves all pages without allocating physical storage.
///
/// The reserved range of pages cannot be used by any other allocation operations until the range
/// of pages is released.
///
/// Reserved pages can be identified in subsequent calls to the [`VirtualAllocExNuma`] function.
/// This attribute is valid only if the `file` parameter is [`INVALID_HANDLE_VALUE`] (that is, a
/// file mapping object that is backed by the system paging file).
pub const SEC_RESERVE: DWORD = 0x04000000;

/// Allocates physical storage in memory or the paging file for all pages.
///
/// This is the default setting.
pub const SEC_COMMIT: DWORD = 0x08000000;

/// Sets all pages to noncachable.
///
/// Applications should not use this flag except when explicitly required for a device. Using the
/// interlocked functions with memory mapped with [`SEC_NOCACHE`] can result in an
/// `EXCEPTION_ILLEGAL_INSTRUCTION` exception.
///
/// [`SEC_NOCACHE`] requires either [`SEC_RESERVE`] or [`SEC_COMMIT`] to be set.
pub const SEC_NOCACHE: DWORD = 0x10000000;

/// Specifies that the file that the hFile parameter specifies is an executable image file that
/// will not be executed and the loaded image file will have no forced integrity checks run.
/// Additionally, mapping a view of a file mapping object created with the [`SEC_IMAGE_NO_EXECUTE`]
/// attribute will not invoke driver callbacks registered using the `PsSetLoadImageNotifyRoutine`
/// kernel API.
///
/// The [`SEC_IMAGE_NO_EXECUTE`] attribute must be combined with the [`PAGE_READONLY`] page
/// protection value. No other attributes are valid with [`SEC_IMAGE_NO_EXECUTE`].
///
/// Windows Server 2008 R2, Windows 7, Windows Server 2008 and Windows Vista:  This value is not
/// supported before Windows Server 2012 and Windows 8.
pub const SEC_IMAGE_NO_EXECUTE: DWORD = 0x11000000;

/// Sets all pages to be write-combined.
///
/// Applications should not use this attribute except when explicitly required for a device. Using
/// the interlocked functions with memory that is mapped with [`SEC_WRITECOMBINE`] can result in an
/// `EXCEPTION_ILLEGAL_INSTRUCTION` exception.
///
/// [`SEC_WRITECOMBINE`] requires either the [`SEC_RESERVE`] or [`SEC_COMMIT`] attribute to be set.
pub const SEC_WRITECOMBINE: DWORD = 0x40000000;

/// Enables large pages to be used when mapping images or backing from the pagefile, but not when
/// mapping data for regular files. Be sure to specify the maximum size of the file mapping object
/// as the minimum size of a large page reported by the [`GetLargePageMinimum`] function and to
/// enable the `SeLockMemoryPrivilege` privilege.
pub const SEC_LARGE_PAGES: DWORD = 0x80000000;
