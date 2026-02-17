use crate::{DWORD, HKEY, LPCWSTR, LPDWORD, LPVOID, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    RegCreateKeyEx, RegCreateKeyTransacted, RegGetValue, RegOpenKeyEx, RegOpenKeyTransacted,
    ERROR_FILE_NOT_FOUND, ERROR_INVALID_PARAMETER, ERROR_MORE_DATA, ERROR_SUCCESS,
    HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE,
    HKEY_PERFORMANCE_DATA, HKEY_PERFORMANCE_NLSTEXT, HKEY_PERFORMANCE_TEXT, HKEY_USERS,
    KEY_QUERY_VALUE, KEY_WOW64_32KEY, KEY_WOW64_64KEY, REG_BINARY, REG_DWORD, REG_EXPAND_SZ,
    REG_MULTI_SZ, REG_NONE, REG_QWORD, REG_SZ, RRF_NOEXPAND, RRF_RT_ANY, RRF_RT_DWORD,
    RRF_RT_QWORD, RRF_RT_REG_BINARY, RRF_RT_REG_DWORD, RRF_RT_REG_EXPAND_SZ, RRF_RT_REG_MULTI_SZ,
    RRF_RT_REG_NONE, RRF_RT_REG_QWORD, RRF_RT_REG_SZ, RRF_SUBKEY_WOW6432KEY, RRF_SUBKEY_WOW6464KEY,
    RRF_ZEROONFAILURE,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "Advapi32")]
unsafe extern "system" {
    /// Retrieves the type and data for the specified registry value
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
    ///    - [`HKEY_PERFORMANCE_NLSTEXT`]
    ///    - [`HKEY_PERFORMANCE_TEXT`]
    ///    - [`HKEY_USERS`]
    ///  * `sub_key` - The path of a registry key relative to the key specified by the `key`
    ///                parameter. The registry value will be retrieved from this subkey. The path
    ///                is not case sensitive. If this parameter is [`null`] or an empty string, "",
    ///                the value will be read from the key specified by `key` itself.
    ///  * `value` - The name of the registry value. If this parameter is [`null`] or an empty
    ///              string, "", the function retrieves the type and data for the key's unnamed or
    ///              default value, if any. Keys do not automatically have an unnamed or default
    ///              value, and unnamed values can be of any type.
    ///  * `flags` - The flags that restrict the data type of value to be queried. If the data type
    ///              of the value does not meet this criteria, the function fails. This parameter
    ///              can be one or more of the following values:
    ///    * [`RRF_RT_ANY`] - No type restriction.
    ///    * [`RRF_RT_DWORD`] - Restrict type to 32-bit `RRF_RT_REG_BINARY | RRF_RT_REG_DWORD`.
    ///    * [`RRF_RT_QWORD`] - Restrict type to 64-bit `RRF_RT_REG_BINARY | RRF_RT_REG_QWORD`.
    ///    * [`RRF_RT_REG_BINARY`] - Restrict type to [`REG_BINARY`].
    ///    * [`RRF_RT_REG_DWORD`] - Restrict type to [`REG_DWORD`].
    ///    * [`RRF_RT_REG_EXPAND_SZ`] - Restrict type to [`REG_EXPAND_SZ`].
    ///    * [`RRF_RT_REG_MULTI_SZ`] - Restrict type to [`REG_MULTI_SZ`].
    ///    * [`RRF_RT_REG_NONE`] - Restrict type to [`REG_NONE`].
    ///    * [`RRF_RT_REG_QWORD`] - Restrict type to [`REG_QWORD`].
    ///    * [`RRF_RT_REG_SZ`] - Restrict type to [`REG_SZ`].
    ///    * [`RRF_NOEXPAND`] - Do not automatically expand environment strings if the value is of
    ///                         type [`REG_EXPAND_SZ`].
    ///    * [`RRF_ZEROONFAILURE`] - If `data` is not [`null_mut`], set the contents of the buffer
    ///                              to zeroes on failure.
    ///    * [`RRF_SUBKEY_WOW6464KEY`] - If `sub_key` is not [`null`], open the subkey that
    ///                                  `sub_key` specifies with the [`KEY_WOW64_64KEY`] access
    ///                                  rights. You cannot specify [`RRF_SUBKEY_WOW6464KEY`] in
    ///                                  combination with [`RRF_SUBKEY_WOW6432KEY`].
    ///    * [`RRF_SUBKEY_WOW6432KEY`] - If `sub_key` is not [`null`], open the subkey that
    ///                                  `sub_key` specifies with the [`KEY_WOW64_32KEY`] access
    ///                                  rights. You cannot specify [`RRF_SUBKEY_WOW6432KEY`] in
    ///                                  combination with [`RRF_SUBKEY_WOW6464KEY`].
    ///  * `r#type` - A pointer to a variable that receives a code indicating the type of data
    ///               stored in the specified value. This parameter can be [`null_mut`] if the type
    ///               is not required.
    ///  * `data` - A pointer to a buffer that receives the value's data. This parameter can be
    ///             [`null_mut`] if the data is not required. If the data is a string, the function
    ///             checks for a terminating null character. If one is not found, the string is
    ///             stored with a null terminator if the buffer is large enough to accommodate the
    ///             extra character. Otherwise, the function fails and returns [`ERROR_MORE_DATA`].
    ///  * `cb_data` - A pointer to a variable that specifies the size of the buffer pointed to by
    ///                the `data` parameter, in bytes. When the function returns, this variable
    ///                contains the size of the data copied to `data`. The `cb_data` parameter can
    ///                be [`null_mut`] only if `data` is [`null_mut`]. If the data has the
    ///                [`REG_SZ`], [`REG_MULTI_SZ`] or [`REG_EXPAND_SZ`] type, this size includes
    ///                any terminating null character or characters. If the buffer specified by
    ///                `data` parameter is not large enough to hold the data, the function returns
    ///                [`ERROR_MORE_DATA`] and stores the required buffer size in the variable
    ///                pointed to by `cb_data`. In this case, the contents of the `data` buffer are
    ///                zeroes if `flags` specifies [`RRF_ZEROONFAILURE`] and undefined otherwise.
    ///                If `data` is [`null_mut`], and `cb_data` is not [`null_mut`], the function
    ///                returns [`ERROR_SUCCESS`] and stores the size of the data, in bytes, in the
    ///                variable pointed to by `cb_data`. This enables an application to determine
    ///                the best way to allocate a buffer for the value's data. If `key` specifies
    ///                [`HKEY_PERFORMANCE_DATA`] and the `data` buffer is not large enough to
    ///                contain all of the returned data, the function returns [`ERROR_MORE_DATA`]
    ///                and the value returned through the `cb_data` parameter is undefined. This is
    ///                because the size of the performance data can change from one call to the
    ///                next. In this case, you must increase the buffer size and call
    ///                [`RegGetValue`] again passing the updated buffer size in the `cb_data`
    ///                parameter. Repeat this until the function succeeds. You need to maintain a
    ///                separate variable to keep track of the buffer size, because the value
    ///                returned by `cb_data` is unpredictable.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a system error code.
    ///
    /// If the `data` buffer is too small to receive the value, the function returns
    /// [`ERROR_MORE_DATA`].
    ///
    /// If the `value` registry value does not exist, the function returns
    /// [`ERROR_FILE_NOT_FOUND`].
    ///
    /// If `flags` specifies a combination of both [`RRF_SUBKEY_WOW6464KEY`] and
    /// [`RRF_SUBKEY_WOW6432KEY`], the function returns [`ERROR_INVALID_PARAMETER`].
    ///
    /// # Remarks
    /// An application typically calls [`RegEnumValue`] to determine the value names and then
    /// [`RegGetValue`] to retrieve the data for the names.
    ///
    /// If the data has the [`REG_SZ`], [`REG_MULTI_SZ`] or [`REG_EXPAND_SZ`] type, and the ANSI
    /// version of this function is used (by explicitly calling [`RegGetValueA`]), this function
    /// converts the stored Unicode string to an ANSI string before copying it to the buffer
    /// pointed to by `data`.
    ///
    /// When calling this function with `key` set to the [`HKEY_PERFORMANCE_DATA`] handle and a
    /// `value` string of a specified object, the returned data structure sometimes has unrequested
    /// objects. Do not be surprised; this is normal behavior. You should always expect to walk the
    /// returned data structure to look for the requested object.
    ///
    /// Note that operations that access certain registry keys are redirected.
    pub fn RegGetValueW(
        key: HKEY,
        sub_key: LPCWSTR,
        value: LPCWSTR,
        flags: DWORD,
        r#type: LPDWORD,
        data: LPVOID,
        cb_data: LPDWORD,
    ) -> LSTATUS;
}
