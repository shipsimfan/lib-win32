use crate::{
    DWORD, HANDLE, HKEY, LPCWSTR, LPDWORD, LPWSTR, LSTATUS, PHKEY, PVOID, REGSAM,
    SECURITY_ATTRIBUTES,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCreateKeyTransacted, RegDeleteKeyEx, RegDeleteKeyTransacted, RegLoadKey,
    RegOpenKeyTransacted, RegSaveKey, RegSetValueEx, ACCESS_SYSTEM_SECURITY, ERROR_SUCCESS,
    FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER,
    HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_CREATE_SUB_KEY, KEY_READ, KEY_WRITE, REG_CREATED_NEW_KEY,
    REG_OPENED_EXISTING_KEY, REG_OPTION_BACKUP_RESTORE, REG_OPTION_NON_VOLATILE,
    REG_OPTION_VOLATILE,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "Advapi32")]
extern "system" {
    /// Creates the specified registry key and associates it with a transaction. If the key already
    /// exists, the function opens it. Note that key names are not case sensitive.
    ///
    /// Applications that back up or restore system state including system files and registry hives
    /// should use the Volume Shadow Copy Service instead of the registry functions.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. The calling process must have
    ///            [`KEY_CREATE_SUB_KEY`] access to the key. Access for key creation is checked
    ///            against the security descriptor of the registry key, not the access mask
    ///            specified when the handle was obtained. Therefore, even if `key` was opened with
    ///            a `desired` of [`KEY_READ`], it can be used in operations that create keys if
    ///            allowed by its security descriptor. This handle is returned by the
    ///            [`RegCreateKeyTransacted`] or [`RegOpenKeyTransacted`] function, or it can be
    ///            one of the following predefined keys:
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
    ///  * `class` - The user-defined class of this key. This parameter may be ignored. This
    ///              parameter can be [`null_mut`].
    ///  * `options` - This parameter can be one of the following values:
    ///    * [`REG_OPTION_BACKUP_RESTORE`] - If this flag is set, the function ignores the
    ///                                      `desired` parameter and attempts to open the key with
    ///                                      the access required to backup or restore the key. If
    ///                                      the calling thread has the "SeBackupPrivilege"
    ///                                      privilege enabled, the key is opened with the
    ///                                      [`ACCESS_SYSTEM_SECURITY`] and [`KEY_READ`] access
    ///                                      rights. If the calling thread has the
    ///                                      "SeRestorePrivilege" privilege enabled, the key is
    ///                                      opened with the [`ACCESS_SYSTEM_SECURITY`] and
    ///                                      [`KEY_WRITE`] access rights. If both privileges are
    ///                                      enabled, the key has the combined access rights for
    ///                                      both privileges.
    ///    * [`REG_OPTION_NON_VOLATILE`] - This key is not volatile; this is the default. The
    ///                                    information is stored in a file and is preserved when
    ///                                    the system is restarted. The [`RegSaveKey`] function
    ///                                    saves keys that are not volatile.
    ///    * [`REG_OPTION_VOLATILE`] - All keys created by the function are volatile. The
    ///                                information is stored in memory and is not preserved when
    ///                                the corresponding registry hive is unloaded. For
    ///                                [`HKEY_LOCAL_MACHINE`], this occurs when the system is shut
    ///                                down. For registry keys loaded by the [`RegLoadKey`]
    ///                                function, this occurs when the corresponding
    ///                                [`RegUnLoadKey`] is performed. The [`RegSaveKey`] function
    ///                                does not save volatile keys. This flag is ignored for keys
    ///                                that already exist.
    ///  * `desired` - A mask that specifies the access rights for the key to be created.
    ///  * `security_attributes` - A pointer to a [`SECURITY_ATTRIBUTES`] structure that determines
    ///                            whether the returned handle can be inherited by child processes.
    ///                            If `security_attributes` is [`null`], the handle cannot be
    ///                            inherited. The `security_attributes` member of the structure
    ///                            specifies a security descriptor for the new key. If
    ///                            `security_attributes` is [`null`], the key gets a default
    ///                            security descriptor. The ACLs in a default security descriptor
    ///                            for a key are inherited from its direct parent key.
    ///  * `result` - A pointer to a variable that receives a handle to the opened or created key.
    ///               If the key is not one of the predefined registry keys, call the
    ///               [`RegCloseKey`] function after you have finished using the handle.
    ///  * `disposition` - A pointer to a variable that receives one of the following disposition
    ///                    values:
    ///    * [`REG_CREATED_NEW_KEY`] - The key did not exist and was created.
    ///    * [`REG_OPENED_EXISTING_KEY`] - The key existed and was simply opened without being
    ///                                    changed.
    ///  * `transaction` - A handle to an active transaction. This handle is returned by the
    ///                    [`CreateTransaction`] function.
    ///  * `extended_parameter` - This parameter is reserved and must be [`null_mut`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// When a key is created using this function, subsequent operations on the key are transacted.
    /// If a non-transacted operation is performed on the key before the transaction is committed,
    /// the transaction is rolled back. After a transaction is committed or rolled back, you must
    /// re-open the key using [`RegCreateKeyTransacted`] or [`RegOpenKeyTransacted`] with an active
    /// transaction handle to make additional operations transacted.
    ///
    /// Note that subsequent operations on subkeys of this key are not automatically transacted.
    /// Therefore, [`RegDeleteKeyEx`] does not perform a transacted delete operation. Instead, use
    /// the [`RegDeleteKeyTransacted`] function to perform a transacted delete operation.
    ///
    /// The key that the [`RegCreateKeyTransacted`] function creates has no values. An application
    /// can use the [`RegSetValueEx`] function to set key values.
    ///
    /// The RegCreateKeyTransacted function creates all missing keys in the specified path. An
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
    pub fn RegCreateKeyTransactedW(
        key: HKEY,
        sub_key: LPCWSTR,
        reserved: DWORD,
        class: LPWSTR,
        options: DWORD,
        desired: REGSAM,
        security_attributes: *const SECURITY_ATTRIBUTES,
        result: PHKEY,
        disposition: LPDWORD,
        transaction: HANDLE,
        extended_parameter: PVOID,
    ) -> LSTATUS;
}
