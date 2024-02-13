use crate::{DWORD, HKEY, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCreateKeyEx, RegOpenKeyEx, RegRestoreKey, RegSaveKey, ERROR_SUCCESS,
    FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER,
    HKEY_LOCAL_MACHINE, HKEY_USERS, REG_FORCE_RESTORE, REG_WHOLE_HIVE_VOLATILE,
};

#[link(name = "Advapi32")]
extern "system" {
    /// Reads the registry information in a specified file and copies it over the specified key.
    /// This registry information may be in the form of a key and multiple levels of subkeys.
    ///
    /// Applications that back up or restore system state including system files and registry hives
    /// should use the Volume Shadow Copy Service instead of the registry functions.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. This handle is returned by the
    ///            [`RegCreateKeyEx`] or [`RegOpenKeyEx`] function. It can also be one of the
    ///            following predefined keys: [`HKEY_CLASSES_ROOT`], [`HKEY_CURRENT_CONFIG`],
    ///            [`HKEY_CURRENT_USER`], [`HKEY_LOCAL_MACHINE`], [`HKEY_USERS`] Any information
    ///            contained in this key and its descendent keys is overwritten by the information
    ///            in the file pointed to by the `file` parameter.
    ///  * `file` - The name of the file with the registry information. This file is typically
    ///             created by using the [`RegSaveKey`] function.
    ///  * `flags` - The flags that indicate how the key or keys are to be restored. This parameter
    ///              can be one of the following values:
    ///    * [`REG_FORCE_RESTORE`] - If specified, the restore operation is executed even if open
    ///                              handles exist at or beneath the location in the registry
    ///                              hierarchy to which the `key` parameter points.
    ///    * [`REG_WHOLE_HIVE_VOLATILE`] - If specified, a new, volatile (memory only) set of
    ///                                    registry information, or hive, is created. If
    ///                                    [`REG_WHOLE_HIVE_VOLATILE`] is specified, the key
    ///                                    identified by the hKey parameter must be either the
    ///                                    [`HKEY_USERS`] or [`HKEY_LOCAL_MACHINE`] value.
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
    /// If any subkeys of the `key` parameter are open, [`RegRestoreKey`] fails.
    ///
    /// The calling process must have the "SE_RESTORE_NAME` and `SE_BACKUP_NAME" privileges on the
    /// computer in which the registry resides.
    ///
    /// This function replaces the keys and values below the specified key with the keys and values
    /// that are subsidiary to the top-level key in the file, no matter what the name of the
    /// top-level key in the file might be. For example, `key` might identify a key A with subkeys
    /// B and C, while the `file` parameter specifies a file containing key X with subkeys Y and Z.
    /// After a call to [`RegRestoreKey`], the registry would contain key A with subkeys Y and Z.
    /// The value entries of A would be replaced by the value entries of X.
    ///
    /// The new information in the file specified by `file` overwrites the contents of the key
    /// specified by the `key` parameter, except for the key name.
    ///
    /// If `key` represents a key in a remote computer, the path described by `file` is relative to
    /// the remote computer.
    pub fn RegRestoreKeyW(key: HKEY, file: LPCWSTR, flags: DWORD) -> LSTATUS;
}
