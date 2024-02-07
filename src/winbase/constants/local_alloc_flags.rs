use crate::UINT;

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
