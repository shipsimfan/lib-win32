use crate::{HKEY, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegConnectRegistry, RegCreateKeyEx, RegReplaceKey, RegSaveKey, ERROR_SUCCESS,
    FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER,
    HKEY_LOCAL_MACHINE, HKEY_USERS,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
unsafe extern "system" {
    /// Replaces the file backing a registry key and all its subkeys with another file, so that
    /// when the system is next started, the key and subkeys will have the values stored in the new
    /// file.
    ///
    /// Applications that back up or restore system state including system files and registry hives
    /// should use the Volume Shadow Copy Service instead of the registry functions.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. This handle is returned by the
    ///            [`RegCreateKeyEx`] or RegOpenKeyEx function, or it can be one of the following
    ///            predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_CONFIG`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_USERS`]
    ///  * `sub_key` - The name of the registry key whose subkeys and values are to be replaced. If
    ///                the key exists, it must be a subkey of the key identified by the `key`
    ///                parameter. If the specified subkey does not exist, it is created. This
    ///                parameter can be [`null`]. If the specified subkey is not the root of a
    ///                hive, [`RegReplaceKey`] traverses up the hive tree structure until it
    ///                encounters a hive root, then it replaces the contents of that hive with the
    ///                contents of the data file specified by `new_file`.
    ///  * `new_file` - The name of the file with the registry information. This file is typically
    ///                 created by using the [`RegSaveKey`] function.
    ///  * `old_file` - The name of the file that receives a backup copy of the registry
    ///                 information being replaced.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// There are two different registry hive file formats. Registry hives created on current
    /// operating systems typically cannot be loaded by earlier ones.
    ///
    /// The file specified by the `new_file` parameter remains open until the system is restarted.
    ///
    /// If `key` is a handle returned by [`RegConnectRegistry`], then the paths specified in
    /// `new_file` and `old_file` are relative to the remote computer.
    ///
    /// The calling process must have the "SE_RESTORE_NAME` and `SE_BACKUP_NAME" privileges on the
    /// computer in which the registry resides.
    pub fn RegReplaceKeyW(
        key: HKEY,
        sub_key: LPCWSTR,
        new_file: LPCWSTR,
        old_file: LPCWSTR,
    ) -> LSTATUS;
}
