use crate::{HKEY, LPBYTE, LPCWSTR, LPDWORD, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    RegCreateKeyEx, RegCreateKeyTransacted, RegEnumValue, RegGetValue, RegOpenKeyEx,
    RegOpenKeyTransacted, RegQueryValueEx, ERROR_FILE_NOT_FOUND, ERROR_MORE_DATA, ERROR_SUCCESS,
    HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    HKEY_PERFORMANCE_DATA, HKEY_PERFORMANCE_NLSTEXT, HKEY_PERFORMANCE_TEXT, HKEY_USERS,
    KEY_QUERY_VALUE, REG_EXPAND_SZ, REG_MULTI_SZ, REG_SZ,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "Advapi32")]
unsafe extern "system" {
    /// Retrieves the type and data for the specified value name associated with an open registry
    /// key.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. The key must have been opened with the
    ///    [`KEY_QUERY_VALUE`] access right. This handle is returned by the [`RegCreateKeyEx`],
    ///    [`RegCreateKeyTransacted`], [`RegOpenKeyEx`], or [`RegOpenKeyTransacted`] function. It
    ///    can also be one of the following predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_CONFIG`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_PERFORMANCE_DATA`]
    ///    * [`HKEY_PERFORMANCE_NLSTEXT`]
    ///    * [`HKEY_PERFORMANCE_TEXT`]
    ///    * [`HKEY_USERS`]
    ///  * `value_name` - The name of the registry value. If `value_name` is [`null`] or an empty
    ///                   string, "", the function retrieves the type and data for the key's
    ///                   unnamed or default value, if any. If `value_name` specifies a value that
    ///                   is not in the registry, the function returns [`ERROR_FILE_NOT_FOUND`].
    ///                   Keys do not automatically have an unnamed or default value. Unnamed
    ///                   values can be of any type.
    ///  * `reserved` - This parameter is reserved and must be [`null_mut`].
    ///  * `r#type` - A pointer to a variable that receives a code indicating the type of data
    ///               stored in the specified value. The `r#type` parameter can be [`null_mut`] if
    ///               the type code is not required.
    ///  * `data` - A pointer to a buffer that receives the value's data. This parameter can be
    ///             [`null_mut`] if the data is not required.
    ///  * `cb_data` - A pointer to a variable that specifies the size of the buffer pointed to by
    ///                the `data` parameter, in bytes. When the function returns, this variable
    ///                contains the size of the data copied to `data`. The `cb_data` parameter can
    ///                be [`null_mut`] only if `data` is [`null_mut`]. If the data has the
    ///                [`REG_SZ`], [`REG_MULTI_SZ`] or [`REG_EXPAND_SZ`] type, this size includes
    ///                any terminating null character or characters unless the data was stored
    ///                without them. If the buffer specified by `data` parameter is not large
    ///                enough to hold the data, the function returns [`ERROR_MORE_DATA`] and stores
    ///                the required buffer size in the variable pointed to by `cb_data`. In this
    ///                case, the contents of the `data` buffer are undefined. If `data` is
    ///                [`null_mut`], and `cb_data` is not [`null_mut`], the function returns
    ///                [`ERROR_SUCCESS`] and stores the size of the data, in bytes, in the variable
    ///                pointed to by `cb_data`. This enables an application to determine the best
    ///                way to allocate a buffer for the value's data. If `key` specifies
    ///                [`HKEY_PERFORMANCE_DATA`] and the `data` buffer is not large enough to
    ///                contain all of the returned data, [`RegQueryValueEx`] returns
    ///                [`ERROR_MORE_DATA`] and the value returned through the `cb_data` parameter
    ///                is undefined. This is because the size of the performance data can change
    ///                from one call to the next. In this case, you must increase the buffer size
    ///                and call [`RegQueryValueEx`] again passing the updated buffer size in the
    ///                `cb_data` parameter. Repeat this until the function succeeds. You need to
    ///                maintain a separate variable to keep track of the buffer size, because the
    ///                value returned by `cb_data` is unpredictable. If the `value_name` registry
    ///                value does not exist, [`RegQueryValueEx`] returns [`ERROR_FILE_NOT_FOUND`]
    ///                and the value returned through the `cb_data` parameter is undefined.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a system error code.
    ///
    /// If the `data` buffer is too small to receive the data, the function returns
    /// [`ERROR_MORE_DATA`].
    ///
    /// If the `value_name` registry value does not exist, the function returns
    /// [`ERROR_FILE_NOT_FOUND`].
    ///
    /// # Remarks
    /// An application typically calls [`RegEnumValue`] to determine the value names and then
    /// [`RegQueryValueEx`] to retrieve the data for the names.
    ///
    /// If the data has the [`REG_SZ`], [`REG_MULTI_SZ`] or [`REG_EXPAND_SZ`] type, the string may
    /// not have been stored with the proper terminating null characters. Therefore, even if the
    /// function returns [`ERROR_SUCCESS`], the application should ensure that the string is
    /// properly terminated before using it; otherwise, it may overwrite a buffer. (Note that
    /// [`REG_MULTI_SZ`] strings should have two terminating null characters.) One way an
    /// application can ensure that the string is properly terminated is to use [`RegGetValue`],
    /// which adds terminating null characters if needed.
    ///
    /// If the data has the [`REG_SZ`], [`REG_MULTI_SZ`] or [`REG_EXPAND_SZ`] type, and the ANSI
    /// version of this function is used (by explicitly calling [`RegQueryValueExA`]), this
    /// function converts the stored Unicode string to an ANSI string before copying it to the
    /// buffer pointed to by `data`.
    ///
    /// When calling the [`RegQueryValueEx`] function with `key` set to the
    /// [`HKEY_PERFORMANCE_DATA`] handle and a value string of a specified object, the returned
    /// data structure sometimes has unrequested objects. Do not be surprised; this is normal
    /// behavior. When calling the [`RegQueryValueEx`] function, you should always expect to walk
    /// the returned data structure to look for the requested object.
    pub fn RegQueryValueExW(
        key: HKEY,
        value_name: LPCWSTR,
        reserved: LPDWORD,
        r#type: LPDWORD,
        data: LPBYTE,
        cb_data: LPDWORD,
    ) -> LSTATUS;
}
