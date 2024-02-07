use crate::HLOCAL;

// rustdoc imports
#[allow(unused_imports)]
use crate::LocalAlloc;
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "system" {
    /// Frees the specified local memory object and invalidates its handle.
    ///
    /// # Parameters
    ///  * `mem` - A handle to the local memory object. This handle is returned by either the
    ///            [`LocalAlloc`] or [`LocalReAlloc`] function. It is not safe to free memory
    ///            allocated with [`GlobalAlloc`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`null_mut`].
    ///
    /// If the function fails, the return value is equal to a handle to the local memory object. To
    /// get extended error information, call [`GetLastError`].
    pub fn LocalFree(mem: HLOCAL) -> HLOCAL;
}
