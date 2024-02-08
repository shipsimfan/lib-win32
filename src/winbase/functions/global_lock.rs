use crate::{HGLOBAL, LPVOID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, GlobalAlloc, GlobalReAlloc, GMEM_FIXED};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Locks a global memory object and returns a pointer to the first byte of the object's memory
    /// block.
    ///
    /// # Parameters
    ///  * `mem` - A handle to the global memory object. This handle is returned by either the
    ///            [`GlobalAlloc`] or [`GlobalReAlloc`] function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a pointer to the first byte of the memory
    /// block.
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// The internal data structures for each memory object include a lock count that is initially
    /// zero. For movable memory objects, [`GlobalLock`] increments the count by one, and the
    /// [`GlobalUnlock`] function decrements the count by one. Each successful call that a process
    /// makes to [`GlobalLock`] for an object must be matched by a corresponding call to
    /// [`GlobalUnlock`]. Locked memory will not be moved or discarded, unless the memory object is
    /// reallocated by using the [`GlobalReAlloc`] function. The memory block of a locked memory
    /// object remains locked until its lock count is decremented to zero, at which time it can be
    /// moved or discarded.
    ///
    /// Memory objects allocated with [`GMEM_FIXED`] always have a lock count of zero. For these
    /// objects, the value of the returned pointer is equal to the value of the specified handle.
    ///
    /// If the specified memory block has been discarded or if the memory block has a zero-byte
    /// size, this function returns [`null_mut`].
    ///
    /// Discarded objects always have a lock count of zero.
    pub fn GlobalLock(mem: HGLOBAL) -> LPVOID;
}
