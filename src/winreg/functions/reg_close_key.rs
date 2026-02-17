use crate::{HKEY, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegConnectRegistry, RegCreateKeyEx, RegCreateKeyTransacted, RegFlushKey,
    RegOpenKeyEx, RegOpenKeyTransacted, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM,
};

#[link(name = "Advapi32")]
unsafe extern "system" {
    /// Closes a handle to the specified registry key.
    ///
    /// # Parameters
    ///  * `key` - A handle to the open key to be closed. The handle must have been opened by the
    ///            [`RegCreateKeyEx`], [`RegCreateKeyTransacted`], [`RegOpenKeyEx`],
    ///            [`RegOpenKeyTransacted`], or [`RegConnectRegistry`] function.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// The handle for a specified key should not be used after it has been closed, because it will
    /// no longer be valid. Key handles should not be left open any longer than necessary.
    ///
    /// The [`RegCloseKey`] function does not necessarily write information to the registry before
    /// returning; it can take as much as several seconds for the cache to be flushed to the hard
    /// disk. If an application must explicitly write registry information to the hard disk, it can
    /// use the [`RegFlushKey`] function. [`RegFlushKey`], however, uses many system resources and
    /// should be called only when necessary.
    pub fn RegCloseKey(key: HKEY) -> LSTATUS;
}
