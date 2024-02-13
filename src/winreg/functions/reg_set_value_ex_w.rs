use crate::{BYTE, DWORD, HKEY, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCreateKeyEx, RegOpenKeyEx, RegSetKeyValue, ERROR_SUCCESS,
    FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER,
    HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_NLSTEXT, HKEY_PERFORMANCE_TEXT, HKEY_USERS, KEY_SET_VALUE,
    REG_EXPAND_SZ, REG_MULTI_SZ, REG_SZ,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
extern "system" {
    /// Sets the data and type of a specified value under a registry key.
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
    ///    * [`HKEY_PERFORMANCE_TEXT`]
    ///    * [`HKEY_PERFORMANCE_NLSTEXT`]
    ///  * `value_name` - The name of the value to be set. If a value with this name is not already
    ///                   present in the key, the function adds it to the key. If `value_name` is
    ///                   [`null`] or an empty string, "", the function sets the type and data for
    ///                   the key's unnamed or default value. Registry keys do not have default
    ///                   values, but they can have one unnamed value, which can be of any type.
    ///  * `reserved` - This parameter is reserved and must be zero.
    ///  * `r#type` - The type of data pointed to by the `data` parameter.
    ///  * `data` - The data to be stored. For string-based types, such as [`REG_SZ`], the string
    ///             must be null-terminated. With the [`REG_MULTI_SZ`] data type, the string must
    ///             be terminated with two null characters.
    ///  * `data_count` - The size of the information pointed to by the `data` parameter, in bytes.
    ///                   If the data is of type [`REG_SZ`], [`REG_EXPAND_SZ`], or
    ///                   [`REG_MULTI_SZ`], `data_count` must include the size of the terminating
    ///                   null character or characters.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// Value sizes are limited by available memory. However, storing large values in the registry
    /// can affect its performance. Long values (more than 2,048 bytes) should be stored as files,
    /// with the locations of the files stored in the registry.
    ///
    /// Application elements such as icons, bitmaps, and executable files should be stored as files
    /// and not be placed in the registry.
    ///
    /// If `type` is the [`REG_SZ`], [`REG_MULTI_SZ`], or [`REG_EXPAND_SZ`] type and the ANSI
    /// version of this function is used, the data pointed to by the `data` parameter must be an
    /// ANSI character string. The string is converted to Unicode before it is stored in the
    /// registry.
    ///
    /// Note that operations that access certain registry keys are redirected.
    ///
    /// Consider using the [`RegSetKeyValue`] function, which provides a more convenient way to set
    /// the value of a registry key.
    pub fn RegSetValueExW(
        key: HKEY,
        value_name: LPCWSTR,
        reserved: DWORD,
        r#type: DWORD,
        data: *const BYTE,
        data_count: DWORD,
    ) -> LSTATUS;
}
