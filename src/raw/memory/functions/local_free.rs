use crate::raw::HLocal;

// rustdoc imports
#[allow(unused_imports)]
use crate::raw::{GetLastError, LocalAlloc};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Kernel32")]
extern "C" {
    /// # LocalFree function (winbase.h)
    ///
    /// Frees the specified local memory object and invalidates its handle.
    ///
    /// ## Parameters
    /// `mem`\
    /// A handle to the local memory object. This handle is returned by either
    /// the [`LocalAlloc`] or [`LocalReAlloc`] function. It is not safe to free
    /// memory allocated with [`GlobalAlloc`].
    ///
    /// ## Return Value
    /// If the function succeeds, the return value is [`null`].
    ///
    /// If the function fails, the return value is equal to a handle to the
    /// local memory object. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// ## Remarks
    /// If the process tries to examine or modify the memory after it has been
    /// freed, heap corruption may occur or an access violation exception
    /// (EXCEPTION_ACCESS_VIOLATION) may be generated.
    ///
    /// If the `mem` parameter is [`null`], [`LocalFree`] ignores the parameter
    /// and returns [`null`].
    ///
    /// The [`LocalFree`] function will free a locked memory object. A locked
    /// memory object has a lock count greater than zero. The [`LocalLock`]
    /// function locks a local memory object and increments the lock count by
    /// one. The [`LocalUnlock`] function unlocks it and decrements the lock
    /// count by one. To get the lock count of a local memory object, use the
    /// [`LocalFlags`] function.
    ///
    /// If an application is running under a debug version of the system,
    /// [`LocalFree`] will issue a message that tells you that a locked object
    /// is being freed. If you are debugging the application, [`LocalFree`]
    /// will enter a breakpoint just before freeing a locked object. This
    /// allows you to verify the intended behavior, then continue execution.
    pub fn LocalFree(mem: HLocal) -> HLocal;
}
