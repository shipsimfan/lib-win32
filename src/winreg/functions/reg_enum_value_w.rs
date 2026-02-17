use crate::{DWORD, HKEY, LPBYTE, LPDWORD, LPWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    RegCreateKeyEx, RegCreateKeyTransacted, RegEnumValue, RegOpenKeyEx, RegOpenKeyTransacted,
    ERROR_MORE_DATA, ERROR_NO_MORE_ITEMS, ERROR_SUCCESS, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG,
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA, HKEY_USERS, KEY_QUERY_VALUE,
    REG_EXPAND_SZ, REG_MULTI_SZ, REG_SZ,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Advapi32")]
unsafe extern "system" {
    /// Enumerates the values for the specified open registry key. The function copies one indexed
    /// value name and data block for the key each time it is called.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. The key must have been opened with the
    ///            [`KEY_QUERY_VALUE`] access right. This handle is returned by the
    ///            [`RegCreateKeyEx`], [`RegCreateKeyTransacted`], [`RegOpenKeyEx`], or
    ///            [`RegOpenKeyTransacted`] function. It can also be one of the following
    ///            predefined keys:
    ///    - [`HKEY_CLASSES_ROOT`]
    ///    - [`HKEY_CURRENT_CONFIG`]
    ///    - [`HKEY_CURRENT_USER`]
    ///    - [`HKEY_LOCAL_MACHINE`]
    ///    - [`HKEY_PERFORMANCE_DATA`]
    ///    - [`HKEY_USERS`]
    ///  * `index` - The index of the value to be retrieved. This parameter should be zero for the
    ///              first call to the [`RegEnumValue`] function and then be incremented for
    ///              subsequent calls. Because values are not ordered, any new value will have an
    ///              arbitrary index. This means that the function may return values in any order.
    ///  * `value_name` - A pointer to a buffer that receives the name of the value as a
    ///                   null-terminated string. This buffer must be large enough to include the
    ///                   terminating null character.
    ///  * `ch_value_name` - A pointer to a variable that specifies the size of the buffer pointed
    ///                      to by the `value_name` parameter, in characters. When the function
    ///                      returns, the variable receives the number of characters stored in the
    ///                      buffer, not including the terminating null character. Registry value
    ///                      names are limited to 32,767 bytes. The ANSI version of this function
    ///                      treats this parameter as a SHORT value. Therefore, if you specify a
    ///                      value greater than 32,767 bytes, there is an overflow and the function
    ///                      may return [`ERROR_MORE_DATA`].
    ///  * `reserved` - This parameter is reserved and must be [`null_mut`].
    ///  * `r#type` - A pointer to a variable that receives a code indicating the type of data
    ///               stored in the specified value. The `r#type` parameter can be [`null_mut`] if
    ///               the type code is not required.
    ///  * `data` - A pointer to a buffer that receives the data for the value entry. This
    ///             parameter can be [`null_mut`] if the data is not required. If `data` is
    ///             [`null_mut`] and `cb_data` is not [`null_mut`], the function stores the size of
    ///             the data, in bytes, in the variable pointed to by `cb_data`. This enables an
    ///             application to determine the best way to allocate a buffer for the data.
    ///  * `cb_data` - A pointer to a variable that specifies the size of the buffer pointed to by
    ///                the `data` parameter, in bytes. When the function returns, the variable
    ///                receives the number of bytes stored in the buffer. This parameter can be
    ///                [`null_mut`] only if `data` is [`null_mut`]. If the data has the [`REG_SZ`],
    ///                [`REG_MULTI_SZ`] or [`REG_EXPAND_SZ`] type, this size includes any
    ///                terminating null character or characters. If the buffer specified by `data`
    ///                is not large enough to hold the data, the function returns
    ///                [`ERROR_MORE_DATA`] and stores the required buffer size in the variable
    ///                pointed to by `cb_data`. In this case, the contents of `data` are undefined.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a system error code. If there are no more values
    /// available, the function returns [`ERROR_NO_MORE_ITEMS`].
    ///
    /// If the `data` buffer is too small to receive the value, the function returns
    /// [`ERROR_MORE_DATA`].
    ///
    /// # Remarks
    /// To enumerate values, an application should initially call the [`RegEnumValue`] function
    /// with the `index` parameter set to zero. The application should then increment `index` and
    /// call the [`RegEnumValue`] function until there are no more values (until the function
    /// returns [`ERROR_NO_MORE_ITEMS`]).
    ///
    /// The application can also set `index` to the index of the last value on the first call to
    /// the function and decrement the index until the value with index 0 is enumerated. To
    /// retrieve the index of the last value, use the [`RegQueryInfoKey`] function.
    ///
    /// While using [`RegEnumValue`], an application should not call any registry functions that
    /// might change the key being queried.
    ///
    /// If the data has the [`REG_SZ`], [`REG_MULTI_SZ`] or [`REG_EXPAND_SZ`] type, the string may
    /// not have been stored with the proper null-terminating characters. Therefore, even if the
    /// function returns [`ERROR_SUCCESS`], the application should ensure that the string is
    /// properly terminated before using it; otherwise, it may overwrite a buffer. (Note that
    /// [`REG_MULTI_SZ`] strings should have two null-terminating characters.)
    ///
    /// To determine the maximum size of the name and data buffers, use the [`RegQueryInfoKey`]
    /// function.
    pub fn RegEnumValueW(
        key: HKEY,
        index: DWORD,
        value_name: LPWSTR,
        ch_value_name: LPDWORD,
        reserved: LPDWORD,
        r#type: LPDWORD,
        data: LPBYTE,
        cb_data: LPDWORD,
    ) -> LSTATUS;
}
