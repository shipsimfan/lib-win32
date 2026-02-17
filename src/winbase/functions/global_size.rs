use crate::{HGLOBAL, SIZE_T};

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, GlobalAlloc, GlobalFlags, GlobalReAlloc};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves the current size of the specified global memory object, in bytes.
    ///
    /// # Parameters
    ///  * `mem` - A handle to the global memory object. This handle is returned by either the
    ///            [`GlobalAlloc`] or [`GlobalReAlloc`] function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is the size of the specified global memory
    /// object, in bytes.
    ///
    /// If the specified handle is not valid or if the object has been discarded, the return value
    /// is zero. To get extended error information, call [`GetLastError`].
    ///
    /// # Remarks
    /// The size of a memory block may be larger than the size requested when the memory was
    /// allocated.
    ///
    /// To verify that the specified object's memory block has not been discarded, use the
    /// [`GlobalFlags`] function before calling [`GlobalSize`].
    pub fn GlobalSize(mem: HGLOBAL) -> SIZE_T;
}
