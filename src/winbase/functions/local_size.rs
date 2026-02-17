use crate::{HLOCAL, SIZE_T};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, LocalAlloc, LocalFlags, LocalHandle, LocalReAlloc};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves the current size of the specified local memory object, in bytes.
    ///
    /// # Parameters
    ///  * `mem` - A handle to the local memory object. This handle is returned by the
    ///            [`LocalAlloc`], [`LocalReAlloc`], or [`LocalHandle`] function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the size of the specified local memory
    /// object, in bytes. If the specified handle is not valid or if the object has been discarded,
    /// the return value is zero. To get extended error information, call [`GetLastError`].
    ///
    /// # Remarks
    /// The size of a memory block may be larger than the size requested when the memory was
    /// allocated.
    ///
    /// To verify that the specified object's memory block has not been discarded, call the
    /// [`LocalFlags`] function before calling [`LocalSize`].
    pub fn LocalSize(mem: HLOCAL) -> SIZE_T;
}
