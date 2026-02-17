use crate::{DWORD, HANDLE, LPGUID, LPSECURITY_ATTRIBUTES, LPWSTR};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CloseHandle, GetLastError, INFINITE, INVALID_HANDLE_VALUE, SECURITY_ATTRIBUTES};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "KtmW32")]
unsafe extern "system" {
    /// Creates a new transaction object.
    ///
    /// # Parameters
    ///  * `transaction_attributes` - A pointer to a [`SECURITY_ATTRIBUTES`] structure that
    ///                               determines whether the returned handle can be inherited by
    ///                               child processes. If this parameter is [`null_mut`], the
    ///                               handle cannot be inherited. The `security_descriptor` member
    ///                               of the structure specifies a security descriptor for the new
    ///                               event. If `transaction_attributes` is [`null_mut`], the
    ///                               object gets a default security descriptor. The access control
    ///                               lists (ACL) in the default security descriptor for a
    ///                               transaction come from the primary or impersonation token of
    ///                               the creator.
    ///  * `uow` - Reserved. Must be zero (0).
    ///  * `create_options` - Any optional transaction instructions.
    ///  * `isolation_level` - Reserved; specify zero (0).
    ///  * `isolation_flags` - Reserved; specify zero (0).
    ///  * `timeout` - The time-out interval, in milliseconds. If a nonzero value is specified, the
    ///                transaction will be aborted when the interval elapses if it has not already
    ///                reached the prepared state. Specify zero (0) or [`INFINITE`] to provide an
    ///                infinite time-out.
    ///  * `description` - A user-readable description of the transaction.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the transaction.
    ///
    /// If the function fails, the return value is [`INVALID_HANDLE_VALUE`]. To get extended error
    /// information, call the [`GetLastError`] function.
    ///
    /// # Remarks
    /// Use the [`CloseHandle`] function to close the transaction handle. If the last transaction
    /// handle is closed before a client calls the [`CommitTransaction`] function with the
    /// transaction handle, then KTM rolls back the transaction.
    ///
    /// If the transaction might need to be promotable to a distributed transaction, then you must
    /// grant the Distributed Transaction Coordinator (DTC) access rights to enlist in the
    /// transaction. To do this, the `transaction_attributes` parameter needs to contain an access
    /// control entry with the DTC’s SID
    /// (S-1-5-80-2818357584-3387065753-4000393942-342927828-138088443) and the
    /// `TRANSACTION_ENLIST` right.
    pub fn CreateTransaction(
        transaction_attributes: LPSECURITY_ATTRIBUTES,
        uow: LPGUID,
        create_options: DWORD,
        isolation_level: DWORD,
        isolation_flags: DWORD,
        timeout: DWORD,
        description: LPWSTR,
    ) -> HANDLE;
}
