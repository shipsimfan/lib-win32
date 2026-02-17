use crate::{HKEY, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCreateKeyEx, RegCreateKeyTransacted, RegOpenKeyEx, RegOpenKeyTransacted,
    DELETE, ERROR_ACCESS_DENIED, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT,
    HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_ENUMERATE_SUB_KEYS,
    KEY_QUERY_VALUE, KEY_SET_VALUE,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
unsafe extern "system" {
    /// Deletes the subkeys and values of the specified key recursively.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. The key must have been opened with the
    ///            following access rights: [`DELETE`], [`KEY_ENUMERATE_SUB_KEYS`], and
    ///            [`KEY_QUERY_VALUE`]. This handle is returned by the [`RegCreateKeyEx`],
    ///            [`RegCreateKeyTransacted`], [`RegOpenKeyEx`], or [`RegOpenKeyTransacted`]
    ///            function, or it can be one of the following predefined Keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_CONFIG`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_USERS`]
    ///  * `sub_key` - The name of the key. This key must be a subkey of the key identified by the
    ///                `key` parameter. If this parameter is [`null`], the subkeys and values of
    ///                `key` are deleted.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// If the key has values, it must be opened with [`KEY_SET_VALUE`] or this function will fail
    /// with [`ERROR_ACCESS_DENIED`].
    pub fn RegDeleteTreeW(key: HKEY, sub_key: LPCWSTR) -> LSTATUS;
}
