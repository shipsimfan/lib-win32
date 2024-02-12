use crate::{DWORD, HKEY, LPCWSTR, LSTATUS, SECURITY_ATTRIBUTES};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCopyTree, RegCreateKeyEx, RegLoadKey, RegReplaceKey, RegRestoreKey,
    RegSaveKey, RegSaveKeyEx, SHCopyKey, ERROR_INVALID_PARAMETER, ERROR_SUCCESS,
    FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT, HKEY_LOCAL_MACHINE, REG_LATEST_FORMAT,
    REG_NO_COMPRESSION, REG_STANDARD_FORMAT,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
extern "system" {
    /// Saves the specified key and all of its subkeys and values to a registry file, in the
    /// specified format.
    ///
    /// Applications that back up or restore system state including system files and registry hives
    /// should use the Volume Shadow Copy Service instead of the registry functions.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. This function does not support the
    ///            [`HKEY_CLASSES_ROOT`] predefined key.
    ///  * `file` - The name of the file in which the specified key and subkeys are to be saved. If
    ///             the file already exists, the function fails. The new file has the archive
    ///             attribute. If the string does not include a path, the file is created in the
    ///             current directory of the calling process for a local key, or in the
    ///             `%systemroot%\system32` directory for a remote key.
    ///  * `security_attributes` - A pointer to a [`SECURITY_ATTRIBUTES`] structure that specifies
    ///                            a security descriptor for the new file. If `security_attributes`
    ///                            is [`null`], the file gets a default security descriptor. The
    ///                            ACLs in a default security descriptor for a file are inherited
    ///                            from its parent directory.
    ///  * `flags` - The format of the saved key or hive. This parameter can be one of the
    ///              following values:
    ///    * [`REG_STANDARD_FORMAT`] - The key or hive is saved in standard format. The standard
    ///                                format is the only format supported by Windows 2000.
    ///    * [`REG_LATEST_FORMAT`] - The key or hive is saved in the latest format. The latest
    ///                              format is supported starting with Windows XP. After the key or
    ///                              hive is saved in this format, it cannot be loaded on an
    ///                              earlier system.
    ///    * [`REG_NO_COMPRESSION`] - The hive is saved with no compression, for faster save
    ///                               operations. The `key` parameter must specify the root of a
    ///                               hive under [`HKEY_LOCAL_MACHINE`] or HKEY_USERS. For example,
    ///                               `HKLM\SOFTWARE` is the root of a hive.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// If more than one of the possible values listed above for the `flags` parameter is specified
    /// in one call to this function—for example, if two or more values are OR'ed— or if
    /// [`REG_NO_COMPRESSION`] is specified and `key` specifies a key that is not the root of a
    /// hive, this function returns [`ERROR_INVALID_PARAMETER`].
    ///
    /// # Remarks
    /// Unlike [`RegSaveKey`], this function does not support the [`HKEY_CLASSES_ROOT`] predefined
    /// key.
    ///
    /// If `key` represents a key on a remote computer, the path described by `file` is relative to
    /// the remote computer.
    ///
    /// The [`RegSaveKeyEx`] function saves only nonvolatile keys. It does not save volatile keys.
    /// A key is made volatile or nonvolatile at its creation; see [`RegCreateKeyEx`].
    ///
    /// You can use the file created by [`RegSaveKeyEx`] in subsequent calls to the [`RegLoadKey`],
    /// [`RegReplaceKey`], or [`RegRestoreKey`] function. If [`RegSaveKeyEx`] fails partway through
    /// its operation, the file will be corrupt and subsequent calls to [`RegLoadKey`],
    /// [`RegReplaceKey`], or [`RegRestoreKey`] for the file will fail.
    ///
    /// Using [`RegSaveKeyEx`] together with [`RegRestoreKey`] to copy subtrees in the registry is
    /// not recommended. This method does not trigger notifications and can invalidate handles used
    /// by other applications. Instead, use the [`SHCopyKey`] function or the [`RegCopyTree`]
    /// function.
    ///
    /// The calling process must have the "SE_BACKUP_NAME" privilege enabled.
    pub fn RegSaveKeyExW(
        key: HKEY,
        file: LPCWSTR,
        security_attributes: *const SECURITY_ATTRIBUTES,
        flags: DWORD,
    ) -> LSTATUS;
}
