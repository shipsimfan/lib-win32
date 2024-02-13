use crate::{DWORD, HANDLE, HKEY, LPCWSTR, LSTATUS, PHKEY, PVOID, REGSAM};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCreateKeyEx, RegOpenCurrentUser, RegOpenKeyEx, RegOpenKeyTransacted,
    ERROR_NO_SYSTEM_RESOURCES, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT,
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
extern "system" {
    /// Opens the specified registry key and associates it with a transaction. Note that key names
    /// are not case sensitive.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. This handle is returned by the
    ///            [`RegCreateKeyEx`], [`RegCreateKeyTransacted`], [`RegOpenKeyEx`], or
    ///            [`RegOpenKeyTransacted`] function. It can also be one of the following
    ///            predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_USERS`]
    ///  * `sub_key` - The name of the registry subkey to be opened. Key names are not case
    ///                sensitive. If the `sub_key` parameter is [`null`] or a pointer to an empty
    ///                string, and if `key` is a predefined key, then the system refreshes the
    ///                predefined key, and `result` receives the same `key` handle passed into the
    ///                function. Otherwise, `result` receives a new handle to the opened key.
    ///  * `options` - This parameter is reserved and must be zero.
    ///  * `desired` - A mask that specifies the desired access rights to the key. The function
    ///                fails if the security descriptor of the key does not permit the requested
    ///                access for the calling process.
    ///  * `result` - A pointer to a variable that receives a handle to the opened key. If the key
    ///               is not one of the predefined registry keys, call the [`RegCloseKey`] function
    ///               after you have finished using the handle.
    ///  * `transaction` - A handle to an active transaction. This handle is returned by the
    ///                    [`CreateTransaction`] function.
    ///  * `extended_parameter` - This parameter is reserved and must be [`null`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// When a key is opened using this function, subsequent operations on the key are transacted.
    /// If a non-transacted operation is performed on the key before the transaction is committed,
    /// the transaction is rolled back. After a transaction is committed or rolled back, you must
    /// re-open the key using the [`RegCreateKeyTransacted`] or [`RegOpenKeyTransacted`] function
    /// with an active transaction handle to make additional operations transacted.
    ///
    /// Note that subsequent operations on subkeys of this key are not automatically transacted.
    /// Therefore, the [`RegDeleteKeyEx`] function does not perform a transacted delete operation.
    /// Instead, use the [`RegDeleteKeyTransacted`] function to perform a transacted delete
    /// operation.
    ///
    /// Unlike the [`RegCreateKeyTransacted`] function, the [`RegOpenKeyTransacted`] function does
    /// not create the specified key if the key does not exist in the registry.
    ///
    /// If your service or application impersonates different users, do not use this function with
    /// [`HKEY_CURRENT_USER`]. Instead, call the [`RegOpenCurrentUser`] function.
    ///
    /// If the key returned in phkResult is a predefined registry key, it is not included in the
    /// provided transaction.
    ///
    /// A single registry key can be opened only 65,534 times. When attempting the 65,535th open
    /// operation, this function fails with [`ERROR_NO_SYSTEM_RESOURCES`].
    pub fn RegOpenKeyTransactedW(
        key: HKEY,
        sub_key: LPCWSTR,
        options: DWORD,
        desired: REGSAM,
        result: PHKEY,
        transaction: HANDLE,
        extended_parameter: PVOID,
    ) -> LSTATUS;
}
