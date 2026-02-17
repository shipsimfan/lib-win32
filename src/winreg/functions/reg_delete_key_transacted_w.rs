use crate::{DWORD, HANDLE, HKEY, LPCWSTR, LSTATUS, PVOID, REGSAM};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    ktmw32::CreateTransaction, FormatMessage, RegCreateKeyEx, RegCreateKeyTransacted,
    RegDeleteKeyTransacted, RegDeleteTree, RegOpenKeyEx, RegOpenKeyTransacted, SHDeleteKey, DELETE,
    ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG,
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
unsafe extern "system" {
    /// Deletes a subkey and its values from the specified platform-specific view of the registry
    /// as a transacted operation. Note that key names are not case sensitive.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. The access rights of this key do not affect
    ///            the delete operation. This handle is returned by the [`RegCreateKeyEx`],
    ///            [`RegCreateKeyTransacted`], [`RegOpenKeyEx`], or [`RegOpenKeyTransacted`]
    ///            function. It can also be one of the following predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_CONFIG`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_USERS`]
    ///  * `sub_key` - The name of the key to be deleted. This key must be a subkey of the key
    ///                specified by the value of the `key` parameter. The function opens the subkey
    ///                with the [`DELETE`] access right. Key names are not case sensitive. The
    ///                value of this parameter cannot be [`null`].
    ///  * `desired` - An access mask the specifies the platform-specific view of the registry.
    ///  * `reserved` - This parameter is reserved and must be zero.
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
    /// A deleted key is not removed until the last handle to it is closed.
    ///
    /// On WOW64, 32-bit applications view a registry tree that is separate from the registry tree
    /// that 64-bit applications view. This function enables an application to delete an entry in
    /// the alternate registry view.
    ///
    /// The subkey to be deleted must not have subkeys. To delete a key and all its subkeys, you
    /// need to enumerate the subkeys and delete them individually. To delete keys recursively, use
    /// the [`RegDeleteTree`] or [`SHDeleteKey`] function.
    ///
    /// If the function succeeds, [`RegDeleteKeyTransacted`] removes the specified key from the
    /// registry. The entire key, including all of its values, is removed. To remove the entire
    /// tree as a transacted operation, use the [`RegDeleteTree`] function with a handle returned
    /// from [`RegCreateKeyTransacted`] or [`RegOpenKeyTransacted`].
    pub fn RegDeleteKeyTransactedW(
        key: HKEY,
        sub_key: LPCWSTR,
        desired: REGSAM,
        reserved: DWORD,
        transaction: HANDLE,
        extended_parameter: PVOID,
    ) -> LSTATUS;
}
