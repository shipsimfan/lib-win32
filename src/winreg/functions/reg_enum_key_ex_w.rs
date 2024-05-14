use crate::{DWORD, HKEY, LPDWORD, LPWSTR, LSTATUS, PFILETIME};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    RegCreateKeyEx, RegCreateKeyTransacted, RegEnumKeyEx, RegOpenKeyEx, RegOpenKeyTransacted,
    ERROR_MORE_DATA, ERROR_NO_MORE_ITEMS, ERROR_SUCCESS, FILETIME, HKEY_CLASSES_ROOT,
    HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA, HKEY_USERS,
    KEY_ENUMERATE_SUB_KEYS,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Advapi32")]
extern "system" {
    /// Enumerates the subkeys of the specified open registry key. The function retrieves
    /// information about one subkey each time it is called.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. The key must have been opened with the
    ///            [`KEY_ENUMERATE_SUB_KEYS`] access right. This handle is returned by the
    ///            [`RegCreateKeyEx`], [`RegCreateKeyTransacted`], [`RegOpenKeyEx`], or
    ///            [`RegOpenKeyTransacted`] function. It can also be one of the following
    ///            predefined keys:
    ///    - [`HKEY_CLASSES_ROOT`]
    ///    - [`HKEY_CURRENT_CONFIG`]
    ///    - [`HKEY_CURRENT_USER`]
    ///    - [`HKEY_LOCAL_MACHINE`]
    ///    - [`HKEY_PERFORMANCE_DATA`]
    ///    - [`HKEY_USERS`]
    ///  * `index` - The index of the subkey to retrieve. This parameter should be zero for the
    ///              first call to the [`RegEnumKeyEx`] function and then incremented for
    ///              subsequent calls. Because subkeys are not ordered, any new subkey will have an
    ///              arbitrary index. This means that the function may return subkeys in any order.
    ///  * `name` - A pointer to a buffer that receives the name of the subkey, including the
    ///             terminating null character. The function copies only the name of the subkey,
    ///             not the full key hierarchy, to the buffer. If the function fails, no
    ///             information is copied to this buffer.
    ///  * `cch_name` - A pointer to a variable that specifies the size of the buffer specified by
    ///                 the `name` parameter, in characters. This size should include the
    ///                 terminating null character. If the function succeeds, the variable pointed
    ///                 to by `name` contains the number of characters stored in the buffer, not
    ///                 including the terminating null character. To determine the required buffer
    ///                 size, use the [`RegQueryInfoKey`] function to determine the size of the
    ///                 largest subkey for the key identified by the `key` parameter.
    ///  * `reserved` - This parameter is reserved and must be [`null_mut`].
    ///  * `class` - A pointer to a buffer that receives the user-defined class of the enumerated
    ///              subkey. This parameter can be [`null_mut`].
    ///  * `cch_class` - A pointer to a variable that specifies the size of the buffer specified by
    ///                  the `class` parameter, in characters. The size should include the
    ///                  terminating null character. If the function succeeds, `class` contains the
    ///                  number of characters stored in the buffer, not including the terminating
    ///                  null character. This parameter can be [`null_mut`] only if `class` is
    ///                  [`null_mut`].
    ///  * `last_write_time` - A pointer to [`FILETIME`] structure that receives the time at which
    ///                        the enumerated subkey was last written. This parameter can be
    ///                        [`null_mut`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a system error code. If there are no more
    /// subkeys available, the function returns [`ERROR_NO_MORE_ITEMS`].
    ///
    /// If the `name` buffer is too small to receive the name of the key, the function returns
    /// [`ERROR_MORE_DATA`].
    ///
    /// # Remarks
    /// To enumerate subkeys, an application should initially call the [`RegEnumKeyEx`] function
    /// with the `index` parameter set to zero. The application should then increment the `index`
    /// parameter and call [`RegEnumKeyEx`] until there are no more subkeys (meaning the function
    /// returns [`ERROR_NO_MORE_ITEMS`]).
    ///
    /// The application can also set `index` to the index of the last subkey on the first call to
    /// the function and decrement the index until the subkey with the index 0 is enumerated. To
    /// retrieve the index of the last subkey, use the [`RegQueryInfoKey`] function.
    ///
    /// While an application is using the [`RegEnumKeyEx`] function, it should not make calls to
    /// any registration functions that might change the key being enumerated.
    ///
    /// Note that operations that access certain registry keys are redirected.
    pub fn RegEnumKeyExW(
        key: HKEY,
        index: DWORD,
        name: LPWSTR,
        cch_name: LPDWORD,
        reserved: LPDWORD,
        class: LPWSTR,
        cch_class: LPDWORD,
        last_write_time: PFILETIME,
    ) -> LSTATUS;
}
