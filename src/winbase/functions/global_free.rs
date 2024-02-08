use crate::HGLOBAL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, GlobalAlloc, GlobalFlags, GlobalLock, GlobalReAlloc, GlobalUnlock, LocalAlloc,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Frees the specified global memory object and invalidates its handle.
    ///
    /// # Parameters
    ///  * `mem` - A handle to the global memory object. This handle is returned by either the
    ///            [`GlobalAlloc`] or [`GlobalReAlloc`] function. It is not safe to free memory
    ///            allocated with [`LocalAlloc`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`null_mut`].
    ///
    /// If the function fails, the return value is equal to a handle to the global memory object.
    /// To get extended error information, call [`GetLastError`].
    ///
    /// # Remarks
    /// If the process examines or modifies the memory after it has been freed, heap corruption may
    /// occur or an access violation exception may be generated.
    ///
    /// The [`GlobalFree`] function will free a locked memory object. A locked memory object has a
    /// lock count greater than zero. The [`GlobalLock`] function locks a global memory object and
    /// increments the lock count by one. The [`GlobalUnlock`] function unlocks it and decrements
    /// the lock count by one. To get the lock count of a global memory object, use the
    /// [`GlobalFlags`] function.
    ///
    /// If an application is running under a debug version of the system, [`GlobalFree`] will issue
    /// a message that tells you that a locked object is being freed. If you are debugging the
    /// application, [`GlobalFree`] will enter a breakpoint just before freeing a locked object.
    /// This allows you to verify the intended behavior, then continue execution.
    pub fn GlobalFree(mem: HGLOBAL) -> HGLOBAL;
}
