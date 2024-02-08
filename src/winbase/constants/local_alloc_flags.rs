use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::{LocalFlags, LocalLock, LocalReAlloc};

/// Allocates fixed memory. The return value is a pointer to the memory object.
pub const LMEM_FIXED: UINT = 0x0000;

/// Same as [`LMEM_FIXED`]
pub const NONZEROLPTR: UINT = LMEM_FIXED;

/// Allocates movable memory. Memory blocks are never moved in physical memory, but they can be
/// moved within the default heap. The return value is a handle to the memory object. To translate
/// the handle to a pointer, use the [`LocalLock`] function. This value cannot be combined with
/// [`LMEM_FIXED`].
pub const LMEM_MOVEABLE: UINT = 0x0002;

/// Same as [`LMEM_MOVEABLE`]
pub const NONZEROLHND: UINT = LMEM_MOVEABLE;

/// Initializes memory contents to zero
pub const LMEM_ZEROINIT: UINT = 0x0040;

/// Combines [`LMEM_FIXED`] and [`LMEM_ZEROINIT`]
pub const LPTR: UINT = 0x0040;

/// Combines [`LMEM_MOVEABLE`] and [`LMEM_ZEROINIT`]
pub const LHND: UINT = 0x0042;

/// If [`LMEM_MODIFY`] is specified, [`LocalReAlloc`] modifies the attributes of the memory object
/// only
pub const LMEM_MODIFY: UINT = 0x0080;

/// Mask for the lock count from [`LocalFlags`]
pub const LMEM_LOCKCOUNT: UINT = 0x00FF;

/// The memory region is marked as discarded
pub const LMEM_DISCARDABLE: UINT = 0x0F00;

/// The local handle is not valid
pub const LMEM_INVALID_HANDLE: UINT = 0x8000;
