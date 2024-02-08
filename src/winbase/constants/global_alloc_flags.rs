use crate::UINT;

/// Allocates fixed memory. The return value is a pointer.
pub const GMEM_FIXED: UINT = 0x0000;

/// Allocates movable memory. Memory blocks are never moved in physical memory, but they can be
/// moved within the default heap. The return value is a handle to the memory object. To translate
/// the handle into a pointer, use the [`GlobalLock`] function. This value cannot be combined with
/// [`GMEM_FIXED`].
pub const GMEM_MOVEABLE: UINT = 0x0002;

/// Initializes memory contents to zero
pub const GMEM_ZEROINIT: UINT = 0x0040;

/// Combines [`GMEM_FIXED`] and [`GMEM_ZEROINIT`].
pub const GPTR: UINT = 0x0040;

/// Combines [`GMEM_MOVEABLE`] and [`GMEM_ZEROINIT`]
pub const GHND: UINT = 0x0042;
