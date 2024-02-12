use crate::{DWORD, HKEY, LPCWSTR, LPDWORD, LPWSTR, LSTATUS, PHKEY, REGSAM, SECURITY_ATTRIBUTES};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
    KEY_CREATE_SUB_KEY, KEY_READ,
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
    ///    * [`REG_OPTION_BACKUP_RESTORE`] -
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
