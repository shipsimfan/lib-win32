use crate::{BOOL, DWORD, HANDLE, PHANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CloseHandle, GetLastError};

#[link(name = "Advapi32")]
extern "system" {
    /// The [`OpenProcessToken`] function opens the access token associated with a process.
    ///
    /// # Parameters
    ///  * `process_handle` - A handle to the process whose access token is opened. The process
    ///                       must have the [`PROCESS_QUERY_LIMITED_INFORMATION`] access
    ///                       permission.
    ///  * `desired_access` - Specifies an access mask that specifies the requested types of access
    ///                       to the access token. These requested access types are compared with
    ///                       the discretionary access control list (DACL) of the token to
    ///                       determine which accesses are granted or denied.
    ///  * `token_handle` - A pointer to a handle that identifies the newly opened access token
    ///                     when the function returns.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// To get a handle to an elevated process from within a non-elevated process, both processes
    /// must be started from the same account.
    ///
    /// If the process being checked was started by a different account, the checking process needs
    /// to have the [`SE_DEBUG_NAME`] privilege enabled.
    ///
    /// To close the access token handle returned through the `token_handle` parameter, call
    /// [`CloseHandle`].
    pub fn OpenProcessToken(
        process_handle: HANDLE,
        desired_access: DWORD,
        token_handle: PHANDLE,
    ) -> BOOL;
}
