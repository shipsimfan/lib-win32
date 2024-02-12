use crate::{DWORD, HKEY, LPCWSTR, LPDWORD, LPWSTR, LSTATUS, PHKEY, REGSAM, SECURITY_ATTRIBUTES};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCreateKeyEx, RegLoadKey, RegOpenCurrentUser, RegSaveKey,
    ACCESS_SYSTEM_SECURITY, DELETE, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT,
    HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_CREATE_SUB_KEY,
    KEY_READ, KEY_WRITE, REG_CREATED_NEW_KEY, REG_OPENED_EXISTING_KEY, REG_OPTION_BACKUP_RESTORE,
    REG_OPTION_CREATE_LINK, REG_OPTION_NON_VOLATILE, REG_OPTION_VOLATILE,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
extern "system" {
    /// Creates the specified registry key. If the key already exists, the function opens it. Note
    /// that key names are not case sensitive.
    ///
    /// To perform transacted registry operations on a key, call the [`RegCreateKeyTransacted`]
    /// function.
    ///
    /// Applications that back up or restore system state including system files and registry hives
    /// should use the Volume Shadow Copy Service instead of the registry functions.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. The calling process must have
    ///            [`KEY_CREATE_SUB_KEY`] access to the key. Access for key creation is checked
    ///            against the security descriptor of the registry key, not the access mask
    ///            specified when the handle was obtained. Therefore, even if hKey was opened with
    ///            a `desired` of [`KEY_READ`], it can be used in operations that modify the
    ///            registry if allowed by its security descriptor. This handle is returned by the
    ///            [`RegCreateKeyEx`] or [`RegOpenKeyEx`] function, or it can be one of the
    ///            following predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_CONFIG`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_USERS`]
    ///  * `sub_key` - The name of a subkey that this function opens or creates. The subkey
    ///                specified must be a subkey of the key identified by the `key` parameter; it
    ///                can be up to 32 levels deep in the registry tree. If `sub_key` is a pointer
    ///                to an empty string, `result` receives a new handle to the key specified by
    ///                `key`. This parameter cannot be [`null`].
    ///  * `reserved` - This parameter is reserved and must be zero.
    ///  * `class` - The user-defined class type of this key. This parameter may be ignored. This
    ///              parameter can be [`null`].
    ///  * `options` - This parameter can be one of the following values:
    ///    * [`REG_OPTION_BACKUP_RESTORE`] - If this flag is set, the function ignores the
    ///                                      `desired` parameter and attempts to open the key with
    ///                                      the access required to backup or restore the key. If
    ///                                      the calling thread has the "SeBackupPrivilege"
    ///                                      privilege enabled, the key is opened with the
    ///                                      [`ACCESS_SYSTEM_SECURITY`] and [`KEY_READ`] access
    ///                                      rights. If the calling thread has the
    ///                                      "SeRestorePrivilege" privilege enabled, beginning with
    ///                                      Windows Vista, the key is opened with the
    ///                                      [`ACCESS_SYSTEM_SECURITY`], [`DELETE`] and
    ///                                      [`KEY_WRITE`] access rights. If both privileges are
    ///                                      enabled, the key has the combined access rights for
    ///                                      both privileges.
    ///    * [`REG_OPTION_CREATE_LINK`] - This key is a symbolic link. The target path is assigned
    ///                                   to the "SymbolicLinkValue" value of the key. The target
    ///                                   path must be an absolute registry path.
    ///    * [`REG_OPTION_NON_VOLATILE`] - This key is not volatile; this is the default. The
    ///                                    information is stored in a file and is preserved when
    ///                                    the system is restarted. The [`RegSaveKey`] function
    ///                                    saves keys that are not volatile.
    ///    * [`REG_OPTION_VOLATILE`] - All keys created by the function are volatile. The
    ///                                information is stored in memory and is not preserved when
    ///                                the corresponding registry hive is unloaded. For
    ///                                [`HKEY_LOCAL_MACHINE`], this occurs only when the system
    ///                                initiates a full shutdown. For registry keys loaded by the
    ///                                [`RegLoadKey`] function, this occurs when the corresponding
    ///                                [`RegUnLoadKey`] is performed. The [`RegSaveKey`] function
    ///                                does not save volatile keys. This flag is ignored for keys
    ///                                that already exist.
    ///  * `desired` - A mask that specifies the access rights for the key to be created.
    ///  * `security_attributes` - A pointer to a [`SECURITY_ATTRIBUTES`] structure that determines
    ///                            whether the returned handle can be inherited by child processes.
    ///                            If `security_attributes` is [`null`], the handle cannot be
    ///                            inherited. The `security_desciptor` member of the structure
    ///                            specifies a security descriptor for the new key. If
    ///                            `security_attributes` is [`null`], the key gets a default
    ///                            security descriptor. The ACLs in a default security descriptor
    ///                            for a key are inherited from its direct parent key.
    ///  * `disposition` - A pointer to a variable that receives one of the following disposition
    ///                    values. If `disposition` is [`null`], no disposition information is
    ///                    returned.
    ///    * [`REG_CREATED_NEW_KEY`] - The key did not exist and was created.
    ///    * [`REG_OPENED_EXISTING_KEY`] - The key existed and was simply opened without being
    ///                                    changed.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// The key that the [`RegCreateKeyEx`] function creates has no values. An application can use
    /// the [`RegSetValueEx`] function to set key values.
    ///
    /// The [`RegCreateKeyEx`] function creates all missing keys in the specified path. An
    /// application can take advantage of this behavior to create several keys at once. For
    /// example, an application can create a subkey four levels deep at the same time as the three
    /// preceding subkeys by specifying a string of the following form for the `sub_key` parameter:
    ///
    /// "subkey1\subkey2\subkey3\subkey4"
    ///
    /// Note that this behavior will result in creation of unwanted keys if an existing key in the
    /// path is spelled incorrectly.
    ///
    /// An application cannot create a key that is a direct child of [`HKEY_USERS`] or
    /// [`HKEY_LOCAL_MACHINE`]. An application can create subkeys in lower levels of the
    /// [`HKEY_USERS`] or [`HKEY_LOCAL_MACHINE`] trees.
    ///
    /// If your service or application impersonates different users, do not use this function with
    /// [`HKEY_CURRENT_USER`]. Instead, call the [`RegOpenCurrentUser`] function.
    ///
    /// Note that operations that access certain registry keys are redirected.
    pub fn RegCreateKeyExW(
        key: HKEY,
        sub_key: LPCWSTR,
        reserved: DWORD,
        class: LPWSTR,
        options: DWORD,
        desired: REGSAM,
        security_attributes: *const SECURITY_ATTRIBUTES,
        result: PHKEY,
        disposition: LPDWORD,
    ) -> LSTATUS;
}
