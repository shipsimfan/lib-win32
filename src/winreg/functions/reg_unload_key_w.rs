use crate::{HKEY, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegConnectRegistry, RegLoadKey, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM,
    HKEY_LOCAL_MACHINE, HKEY_USERS,
};

#[link(name = "Advapi32")]
extern "system" {
    /// Unloads the specified registry key and its subkeys from the registry.
    ///
    /// Applications that back up or restore system state including system files and registry hives
    /// should use the Volume Shadow Copy Service instead of the registry functions.
    ///
    /// # Parameters
    ///  * `key` - A handle to the registry key to be unloaded. This parameter can be a handle
    ///            returned by a call to [`RegConnectRegistry`] function or one of the following
    ///            predefined handles:
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_USERS`]
    ///  * `sub_key` - The name of the subkey to be unloaded. The key referred to by the `sub_key`
    ///                parameter must have been created by using the [`RegLoadKey`] function. Key
    ///                names are not case sensitive.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// This function removes a hive from the registry but does not modify the file containing the
    /// registry information. A hive is a discrete body of keys, subkeys, and values that is rooted
    /// at the top of the registry hierarchy.
    ///
    /// The calling process must have the "SeRestorePrivilege" and "SeBackupPrivilege" privileges
    /// on the computer in which the registry resides.
    pub fn RegUnLoadKeyW(key: HKEY, sub_key: LPCWSTR) -> LSTATUS;
}
