use crate::{HLOCAL, LPVOID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, LocalAlloc, LocalReAlloc, LocalUnlock, LMEM_FIXED};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Locks a local memory object and returns a pointer to the first byte of the object's memory
    /// block.
    ///
    /// # Parameters
    ///  * `mem` - A handle to the local memory object. This handle is returned by either the
    ///            [`LocalAlloc`] or [`LocalReAlloc`] function.
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
    /// zero. For movable memory objects, [`LocalLock`] increments the count by one, and the
    /// [`LocalUnlock`] function decrements the count by one. Each successful call that a process
    /// makes to [`LocalLock`] for an object must be matched by a corresponding call to
    /// [`LocalUnlock`]. Locked memory will not be moved or discarded unless the memory object is
    /// reallocated by using the [`LocalReAlloc`] function. The memory block of a locked memory
    /// object remains locked in memory until its lock count is decremented to zero, at which time
    /// it can be moved or discarded.
    ///
    /// Memory objects allocated with [`LMEM_FIXED`] always have a lock count of zero. For these
    /// objects, the value of the returned pointer is equal to the value of the specified handle.
    ///
    /// If the specified memory block has been discarded or if the memory block has a zero-byte
    /// size, this function returns [`null_mut`].
    ///
    /// Discarded objects always have a lock count of zero.
    pub fn LocalLock(mem: HLOCAL) -> LPVOID;
}
