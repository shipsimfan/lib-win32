use crate::{BOOL, HLOCAL};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, LocalAlloc, LocalLock, LocalReAlloc, ERROR_NOT_LOCKED, FALSE, LMEM_FIXED,
    LMEM_MOVEABLE, NO_ERROR,
};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Decrements the lock count associated with a memory object that was allocated with
    /// [`LMEM_MOVEABLE`]. This function has no effect on memory objects allocated with
    /// [`LMEM_FIXED`].
    ///
    /// # Parameters
    ///  * `mem` - A handle to the local memory object. This handle is returned by either the
    ///            [`LocalAlloc`] or [`LocalReAlloc`] function.
    ///
    /// # Return Value
    /// If the memory object is still locked after decrementing the lock count, the return value is
    /// nonzero. If the memory object is unlocked after decrementing the lock count, the function
    /// returns zero and [`GetLastError`] returns [`NO_ERROR`].
    ///
    /// If the function fails, the return value is zero and [`GetLastError`] returns a value other
    /// than [`NO_ERROR`].
    ///
    /// # Remarks
    /// The internal data structures for each memory object include a lock count that is initially
    /// zero. For movable memory objects, the [`LocalLock`] function increments the count by one,
    /// and [`LocalUnlock`] decrements the count by one. For each call that a process makes to
    /// [`LocalLock`] for an object, it must eventually call [`LocalUnlock`]. Locked memory will
    /// not be moved or discarded unless the memory object is reallocated by using the
    /// [`LocalReAlloc`] function. The memory block of a locked memory object remains locked until
    /// its lock count is decremented to zero, at which time it can be moved or discarded.
    ///
    /// If the memory object is already unlocked, [`LocalUnlock`] returns [`FALSE`] and
    /// [`GetLastError`] reports [`ERROR_NOT_LOCKED`]. Memory objects allocated with [`LMEM_FIXED`]
    /// always have a lock count of zero and cause the [`ERROR_NOT_LOCKED`] error.
    ///
    /// A process should not rely on the return value to determine the number of times it must
    /// subsequently call [`LocalUnlock`] for the memory block.
    pub fn LocalUnlock(mem: HLOCAL) -> BOOL;
}
