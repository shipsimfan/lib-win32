use crate::{HKEY, LPCWSTR, LSTATUS, PHKEY};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCloseKey, RegConnectRegistry, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM,
    HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA, HKEY_USERS, LOGON32_LOGON_NEW_CREDENTIALS,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
unsafe extern "system" {
    /// Establishes a connection to a predefined registry key on another computer.
    ///
    /// # Parameters
    ///  * `machine_name` - The name of the remote computer. The string has the following form:
    ///                     "\\computername". The caller must have access to the remote computer or
    ///                     the function fails. If this parameter is [`null`], the local computer
    ///                     name is used.
    ///  * `key` - A predefined registry handle. This parameter can be one of the following
    ///            predefined keys on the remote computer.
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_PERFORMANCE_DATA`]
    ///    * [`HKEY_USERS`]
    ///  * `result` - A pointer to a variable that receives a key handle identifying the predefined
    ///               handle on the remote computer.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// [`RegConnectRegistry`] requires the Remote Registry service to be running on the remote
    /// computer. By default, this service is configured to be started manually. To configure the
    /// Remote Registry service to start automatically, run Services.msc and change the Startup
    /// Type of the service to Automatic.
    ///
    /// Windows Server 2003 and Windows XP/2000:  The Remote Registry service is configured to
    /// start automatically by default.
    ///
    /// When a handle returned by [`RegConnectRegistry`] is no longer needed, it should be closed
    /// by calling [`RegCloseKey`].
    ///
    /// If the computer is joined to a workgroup and the "Force network logons using local accounts
    /// to authenticate as Guest" policy is enabled, the function fails. Note that this policy is
    /// enabled by default if the computer is joined to a workgroup.
    ///
    /// If the current user does not have proper access to the remote computer, the call to
    /// [`RegConnectRegistry`] fails. To connect to a remote registry, call [`LogonUser`] with
    /// [`LOGON32_LOGON_NEW_CREDENTIALS`] and [`ImpersonateLoggedOnUser`] before calling
    /// [`RegConnectRegistry`].
    ///
    /// Windows 2000: One possible workaround is to establish a session to an administrative share
    /// such as IPC$ using a different set of credentials. To specify credentials other than those
    /// of the current user, use the [`WNetAddConnection2`] function to connect to the share. When
    /// you have finished accessing the registry, cancel the connection.
    ///
    /// Windows XP Home Edition: You cannot use this function to connect to a remote computer
    /// running Windows XP Home Edition. This function does work with the name of the local
    /// computer even if it is running Windows XP Home Edition because this bypasses the
    /// authentication layer.
    pub fn RegConnectRegistryW(machine_name: LPCWSTR, key: HKEY, result: PHKEY) -> LSTATUS;
}
