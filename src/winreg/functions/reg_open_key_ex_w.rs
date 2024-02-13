use crate::{DWORD, HKEY, LPCWSTR, LSTATUS, PHKEY, REGSAM};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCreateKeyEx, RegOpenCurrentUser, RegOpenKeyEx, RegOpenKeyTransacted,
    RegSetValueEx, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT,
    HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_READ,
    KEY_SET_VALUE, REG_OPTION_OPEN_LINK,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
extern "system" {
    /// Opens the specified registry key. Note that key names are not case sensitive.
    ///
    /// To perform transacted registry operations on a key, call the [`RegOpenKeyTransacted`]
    /// function.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. This handle is returned by the
    ///            [`RegCreateKeyEx`] or [`RegOpenKeyEx`] function, or it can be one of the
    ///            following predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_CONFIG`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_USERS`]
    ///  * `sub_key` - The name of the registry subkey to be opened. Key names are not case
    ///                sensitive. If the `sub_key` parameter is [`null`] or a pointer to an empty
    ///                string, and if `key` is a predefined key, then the system refreshes the
    ///                predefined key, and `result` receives the same `key` handle passed into the
    ///                function. Otherwise, `result` receives a new handle to the opened key.
    ///  * `options` - Specifies the option to apply when opening the key. Set this parameter to
    ///                zero or the following:
    ///    * [`REG_OPTION_OPEN_LINK`] - The key is a symbolic link. Registry symbolic links should
    ///                                 only be used when absolutely necessary.
    ///  * `desired` - A mask that specifies the desired access rights to the key to be opened. The
    ///                function fails if the security descriptor of the key does not permit the
    ///                requested access for the calling process.
    ///  * `result` - A pointer to a variable that receives a handle to the opened key. If the key
    ///               is not one of the predefined registry keys, call the [`RegCloseKey`] function
    ///               after you have finished using the handle.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// Unlike the [`RegCreateKeyEx`] function, the [`RegOpenKeyEx`] function does not create the
    /// specified key if the key does not exist in the registry.
    ///
    /// Certain registry operations perform access checks against the security descriptor of the
    /// key, not the access mask specified when the handle to the key was obtained. For example,
    /// even if a key is opened with a `desired` of [`KEY_READ`], it can be used to create registry
    /// keys if the key's security descriptor permits. In contrast, the [`RegSetValueEx`] function
    /// specifically requires that the key be opened with the [`KEY_SET_VALUE`] access right.
    ///
    /// If your service or application impersonates different users, do not use this function with
    /// [`HKEY_CURRENT_USER`]. Instead, call the [`RegOpenCurrentUser`] function.
    ///
    /// Note that operations that access certain registry keys are redirected.
    pub fn RegOpenKeyExW(
        key: HKEY,
        sub_key: LPCWSTR,
        options: DWORD,
        desired: REGSAM,
        result: PHKEY,
    ) -> LSTATUS;
}
