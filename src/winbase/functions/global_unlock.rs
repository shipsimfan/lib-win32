use crate::{BOOL, HGLOBAL};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, GlobalAlloc, GlobalLock, GlobalReAlloc, ERROR_NOT_LOCKED, FALSE, GMEM_FIXED,
    GMEM_MOVEABLE, NO_ERROR, TRUE,
};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Decrements the lock count associated with a memory object that was allocated with
    /// [`GMEM_MOVEABLE`]. This function has no effect on memory objects allocated with
    /// [`GMEM_FIXED`].
    ///
    /// # Parameters
    ///  * `mem` - A handle to the global memory object. This handle is returned by either the
    ///            [`GlobalAlloc`] or [`GlobalReAlloc`] function.
    ///
    /// # Return Value
    /// If the memory object is still locked after decrementing the lock count, the return value is
    /// a nonzero value. If the memory object is unlocked after decrementing the lock count, the
    /// function returns zero and [`GetLastError`] returns [`NO_ERROR`].
    ///
    /// If the function fails, the return value is zero and [`GetLastError`] returns a value other
    /// than [`NO_ERROR`].
    ///
    /// # Remarks
    /// The internal data structures for each memory object include a lock count that is initially
    /// zero. For movable memory objects, the [`GlobalLock`] function increments the count by one,
    /// and [`GlobalUnlock`] decrements the count by one. For each call that a process makes to
    /// [`GlobalLock`] for an object, it must eventually call [`GlobalUnlock`]. Locked memory will
    /// not be moved or discarded, unless the memory object is reallocated by using the
    /// [`GlobalReAlloc`] function. The memory block of a locked memory object remains locked until
    /// its lock count is decremented to zero, at which time it can be moved or discarded.
    ///
    /// Memory objects allocated with [`GMEM_FIXED`] always have a lock count of zero. If the
    /// specified memory block is fixed memory, this function returns [`TRUE`].
    ///
    /// If the memory object is already unlocked, [`GlobalUnlock`] returns [`FALSE`] and
    /// [`GetLastError`] reports [`ERROR_NOT_LOCKED`].
    ///
    /// A process should not rely on the return value to determine the number of times it must
    /// subsequently call [`GlobalUnlock`] for a memory object.
    pub fn GlobalUnlock(mem: HGLOBAL) -> BOOL;
}
