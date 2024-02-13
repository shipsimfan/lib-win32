use crate::{DWORD, HKEY, LPCVOID, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCreateKeyEx, RegCreateKeyTransacted, RegOpenKeyEx, RegOpenKeyTransacted,
    ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG,
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS, KEY_SET_VALUE, REG_EXPAND_SZ, REG_MULTI_SZ,
    REG_SZ,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
extern "system" {
    /// Sets the data for the specified value in the specified registry key and subkey.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. The key must have been opened with the
    ///            [`KEY_SET_VALUE`] access right. This handle is returned by the
    ///            [`RegCreateKeyEx`], [`RegCreateKeyTransacted`], [`RegOpenKeyEx`], or
    ///            [`RegOpenKeyTransacted`] function. It can also be one of the following
    ///            predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_CONFIG`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_USERS`]
    ///  * `sub_key` - The name of the subkey relative to the key identified by `key`. If the
    ///                subkey does not exist, it is created as a non-volatile key with a default
    ///                security descriptor. If this parameter is [`null`], then the value is
    ///                created in the key specified by `key`.
    ///  * `value_name` - The name of the registry value whose data is to be updated.
    ///  * `r#type` - The type of data pointed to by the `data` parameter.
    ///  * `data` - The data to be stored with the specified value name. For string-based types,
    ///             such as [`REG_SZ`], the string must be null-terminated. With the
    ///             [`REG_MULTI_SZ`] data type, the string must be terminated with two null
    ///             characters.
    ///  * `data_count` - The size of the information pointed to by the `data` parameter, in bytes.
    ///                   If the data is of type [`REG_SZ`], [`REG_EXPAND_SZ`], or
    ///                   [`REG_MULTI_SZ`], cbData must include the size of the terminating null
    ///                   character or characters.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    pub fn RegSetKeyValueW(
        key: HKEY,
        sub_key: LPCWSTR,
        value_name: LPCWSTR,
        r#type: DWORD,
        data: LPCVOID,
        data_count: DWORD,
    ) -> LSTATUS;
}
