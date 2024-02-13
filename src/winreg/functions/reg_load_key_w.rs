use crate::{HKEY, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegConnectRegistry, RegLoadAppKey, RegSaveKey, ERROR_SUCCESS,
    FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    HKEY_USERS,
};

#[link(name = "Advapi32")]
extern "system" {
    /// Creates a subkey under [`HKEY_USERS`] or [`HKEY_LOCAL_MACHINE`] and loads the data from the
    /// specified registry hive into that subkey.
    ///
    /// Applications that back up or restore system state including system files and registry hives
    /// should use the Volume Shadow Copy Service instead of the registry functions.
    ///
    /// # Parameters
    ///  * `key` - A handle to the key where the subkey will be created. This can be a handle
    ///            returned by a call to [`RegConnectRegistry`], or one of the following predefined
    ///            handles: [`HKEY_LOCAL_MACHINE`] or [`HKEY_USERS`]. This function always loads
    ///            information at the top of the registry hierarchy. The [`HKEY_CLASSES_ROOT`] and
    ///            [`HKEY_CURRENT_USER`] handle values cannot be specified for this parameter,
    ///            because they represent subsets of the [`HKEY_LOCAL_MACHINE`] and [`HKEY_USERS`]
    ///            handle values, respectively.
    ///  * `sub_key` - The name of the key to be created under `key`. This subkey is where the
    ///                registration information from the file will be loaded. Key names are not
    ///                case sensitive.
    ///  * `file` - The name of the file containing the registry data. This file must be a local
    ///             file that was created with the [`RegSaveKey`] function. If this file does not
    ///             exist, a file is created with the specified name.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// There are two registry hive file formats. Registry hives created on current operating
    /// systems typically cannot be loaded by earlier ones.
    ///
    /// If `key` is a handle returned by [`RegConnectRegistry`], then the path specified in `file`
    /// is relative to the remote computer.
    ///
    /// The calling process must have the "SE_RESTORE_NAME` and `SE_BACKUP_NAME" privileges on the
    /// computer in which the registry resides. To load a hive without requiring these special
    /// privileges, use the [`RegLoadAppKey`] function.
    pub fn RegLoadKeyW(key: HKEY, sub_key: LPCWSTR, file: LPCWSTR) -> LSTATUS;
}
