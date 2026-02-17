use crate::{HGLOBAL, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, GlobalAlloc, GlobalReAlloc, GMEM_DISCARDED, GMEM_FIXED, GMEM_INVALID_HANDLE,
    GMEM_LOCKCOUNT,
};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves information about the specified global memory object.
    ///
    /// # Parameters
    ///  * `mem` - A handle to the global memory object. This handle is returned by either the
    ///            [`GlobalAlloc`] or [`GlobalReAlloc`] function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value specifies the allocation values and the lock
    /// count for the memory object.
    ///
    /// If the function fails, the return value is [`GMEM_INVALID_HANDLE`], indicating that the
    /// global handle is not valid. To get extended error information, call [`GetLastError`].
    ///
    /// # Remarks
    /// The low-order byte of the low-order word of the return value contains the lock count of the
    /// object. To retrieve the lock count from the return value, use the [`GMEM_LOCKCOUNT`] mask
    /// with the bitwise AND (&) operator. The lock count of memory objects allocated with
    /// [`GMEM_FIXED`] is always zero.
    ///
    /// The high-order byte of the low-order word of the return value indicates the allocation
    /// values of the memory object. It can be zero or [`GMEM_DISCARDED`].
    ///
    /// The global functions have greater overhead and provide fewer features than other memory
    /// management functions. New applications should use the heap functions unless documentation
    /// states that a global function should be used.
    pub fn GlobalFlags(mem: HGLOBAL) -> UINT;
}
