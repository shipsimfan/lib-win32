use crate::{HKEY, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegDeleteTree, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT,
    HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA, HKEY_USERS,
};

#[link(name = "Shlwapi")]
extern "system" {
    /// Deletes a subkey and all its descendants. This function removes the key and all the key's
    /// values from the registry.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key, or one of the following predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_CONFIG`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_PERFORMANCE_DATA`]
    ///    * [`HKEY_USERS`]
    ///  * `sub_key` - The address of a null-terminated string specifying the name of the key to
    ///                delete.
    ///
    /// # Return Value
    /// Returns [`ERROR_SUCCESS`] if successful, or a nonzero error code otherwise. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to retrieve a
    /// generic description of the error.
    ///
    /// # Remarks
    /// Alternatively, use the [`RegDeleteKey`] or [`RegDeleteTree`] function.
    pub fn SHDeleteKeyW(key: HKEY, sub_key: LPCWSTR) -> LSTATUS;
}
