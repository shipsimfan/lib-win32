use crate::{HLOCAL, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, LocalAlloc, LocalReAlloc, LMEM_DISCARDABLE, LMEM_FIXED, LMEM_INVALID_HANDLE,
    LMEM_LOCKCOUNT,
};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves information about the specified local memory object.
    ///
    /// # Parameters
    ///  * `mem` - A handle to the local memory object. This handle is returned by either the
    ///            [`LocalAlloc`] or [`LocalReAlloc`] function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value specifies the allocation values and the lock
    /// count for the memory object.
    ///
    /// If the function fails, the return value is [`LMEM_INVALID_HANDLE`], indicating that the
    /// local handle is not valid. To get extended error information, call [`GetLastError`].
    ///
    /// # Remarks
    /// The low-order byte of the low-order word of the return value contains the lock count of the
    /// object. To retrieve the lock count from the return value, use the [`LMEM_LOCKCOUNT`] mask
    /// with the bitwise AND (&) operator. The lock count of memory objects allocated with
    /// [`LMEM_FIXED`] is always zero.
    ///
    /// The high-order byte of the low-order word of the return value indicates the allocation
    /// values of the memory object. It can be zero or [`LMEM_DISCARDABLE`].
    ///
    /// The local functions have greater overhead and provide fewer features than other memory
    /// management functions. New applications should use the heap functions unless documentation
    /// states that a local function should be used.
    pub fn LocalFlags(mem: HLOCAL) -> UINT;
}
