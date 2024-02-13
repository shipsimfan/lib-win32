use crate::{DWORD, LPCWSTR, LSTATUS, PHKEY, REGSAM};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCloseKey, RegLoadAppKey, RegLoadKey, RegSaveKey, RegSaveKeyEx,
    RegSetKeySecurity, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_LOCAL_MACHINE, HKEY_USERS,
    REG_PROCESS_APPKEY,
};

#[link(name = "Advapi32")]
extern "system" {
    /// Loads the specified registry hive as an application hive.
    ///
    /// # Parameters
    ///  * `file` - The name of the hive file. This hive must have been created with the
    ///             [`RegSaveKey`] or [`RegSaveKeyEx`] function. If the file does not exist, an
    ///             empty hive file is created with the specified name.
    ///  * `result` - Pointer to the handle for the root key of the loaded hive. The only way to
    ///               access keys in the hive is through this handle. The registry will prevent an
    ///               application from accessing keys in this hive using an absolute path to the
    ///               key. As a result, it is not possible to navigate to this hive through the
    ///               registry's namespace.
    ///  * `desired` - A mask that specifies the access rights requested for the returned root key.
    ///  * `options` - If this parameter is [`REG_PROCESS_APPKEY`], the hive cannot be loaded again
    ///                while it is loaded by the caller. This prevents access to this registry hive
    ///                by another caller.
    ///  * `reserved` - This parameter is reserved.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// Unlike [`RegLoadKey`], [`RegLoadAppKey`] does not load the hive under
    /// [`HKEY_LOCAL_MACHINE`] or [`HKEY_USERS`]. Instead, the hive is loaded under a special root
    /// that cannot be enumerated. As a result, there is no way to enumerate hives currently loaded
    /// by [`RegLoadAppKey`]. All operations on hives loaded by [`RegLoadAppKey`] have to be
    /// performed relative to the handle returned in `result`.
    ///
    /// If two processes are required to perform operations on the same hive, each process must
    /// call [`RegLoadAppKey`] to retrieve a handle. During the [`RegLoadAppKey`] operation, the
    /// registry will verify if the file has already been loaded. If it has been loaded, the
    /// registry will return a handle to the previously loaded hive rather than re-loading the
    /// hive.
    ///
    /// All keys inside the hive must have the same security descriptor, otherwise the function
    /// will fail. This security descriptor must grant the caller the access specified by the
    /// `desired` parameter or the function will fail. You cannot use the [`RegSetKeySecurity`]
    /// function on any key inside the hive.
    ///
    /// In Windows 8 and later, each process can call [`RegLoadAppKey`] to load multiple hives. In
    /// Windows 7 and earlier, each process can load only one hive using [`RegLoadAppKey`] at a
    /// time.
    ///
    /// Any hive loaded using [`RegLoadAppKey`] is automatically unloaded when all handles to the
    /// keys inside the hive are closed using [`RegCloseKey`].
    pub fn RegLoadAppKeyW(
        file: LPCWSTR,
        result: PHKEY,
        desired: REGSAM,
        options: DWORD,
        reserved: DWORD,
    ) -> LSTATUS;
}
