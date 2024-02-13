use crate::{HKEY, LPCWSTR, LSTATUS, SECURITY_ATTRIBUTES};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCopyTree, RegCreateKeyEx, RegLoadKey, RegReplaceKey, RegRestoreKey,
    RegSaveKey, RegSaveKeyEx, SHCopyKey, ERROR_ALREADY_EXISTS, ERROR_SUCCESS,
    FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT, HKEY_CURRENT_USER,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
extern "system" {
    /// Saves the specified key and all of its subkeys and values to a new file, in the standard
    /// format.
    ///
    /// To specify the format for the saved key or hive, use the [`RegSaveKeyEx`] function.
    ///
    /// Applications that back up or restore system state including system files and registry hives
    /// should use the Volume Shadow Copy Service instead of the registry functions.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. This handle is returned by the
    ///            [`RegCreateKeyEx`] or [`RegOpenKeyEx`] function, or it can be one of the
    ///            following predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_USER`]
    ///  * `file` - The name of the file in which the specified key and subkeys are to be saved. If
    ///             the file already exists, the function fails. If the string does not include a
    ///             path, the file is created in the current directory of the calling process for a
    ///             local key, or in the `%systemroot%\system32` directory for a remote key. The
    ///             new file has the archive attribute.
    ///  * `security_attributes` - A pointer to a [`SECURITY_ATTRIBUTES`] structure that specifies
    ///                            a security descriptor for the new file. If `security_attributes`
    ///                            is [`null`], the file gets a default security descriptor. The
    ///                            ACLs in a default security descriptor for a file are inherited
    ///                            from its parent directory.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// If the file already exists, the function fails with the [`ERROR_ALREADY_EXISTS`] error.
    ///
    /// # Remarks
    /// If `key` represents a key on a remote computer, the path described by `file` is relative to
    /// the remote computer.
    ///
    /// The [`RegSaveKey`] function saves only nonvolatile keys. It does not save volatile keys. A
    /// key is made volatile or nonvolatile at its creation; see [`RegCreateKeyEx`].
    ///
    /// You can use the file created by [`RegSaveKey`] in subsequent calls to the [`RegLoadKey`],
    /// [`RegReplaceKey`], or [`RegRestoreKey`] functions. If [`RegSaveKey`] fails part way through
    /// its operation, the file will be corrupt and subsequent calls to [`RegLoadKey`],
    /// [`RegReplaceKey`], or [`RegRestoreKey`] for the file will fail.
    ///
    /// Using [`RegSaveKey`] together with [`RegRestoreKey`] to copy subtrees in the registry is
    /// not recommended. This method does not trigger notifications and can invalidate handles used
    /// by other applications. Instead, use the [`SHCopyKey`] function or the [`RegCopyTree`]
    /// function.
    ///
    /// The calling process must have the "SeBackupPrivilege" privilege enabled.
    pub fn RegSaveKeyW(
        key: HKEY,
        file: LPCWSTR,
        security_attributes: *const SECURITY_ATTRIBUTES,
    ) -> LSTATUS;
}
