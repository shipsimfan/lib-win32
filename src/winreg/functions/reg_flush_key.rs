use crate::{HKEY, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCloseKey, RegCreateKeyEx, RegCreateKeyTransacted, RegOpenKeyEx,
    RegOpenKeyTransacted, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT,
    HKEY_CURRENT_CONFIG, HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_PERFORMANCE_DATA, HKEY_USERS,
    KEY_QUERY_VALUE,
};

#[link(name = "Advapi32")]
extern "system" {
    /// Writes all the attributes of the specified open registry key into the registry.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. The key must have been opened with the
    ///            [`KEY_QUERY_VALUE`] access right. This handle is returned by the
    ///            [`RegCreateKeyEx`], [`RegCreateKeyTransacted`], [`RegOpenKeyEx`], or
    ///            [`RegOpenKeyTransacted`] function. It can also be one of the following
    ///            predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_CONFIG`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_PERFORMANCE_DATA`]
    ///    * [`HKEY_USERS`]
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// Calling [`RegFlushKey`] is an expensive operation that significantly affects system-wide
    /// performance as it consumes disk bandwidth and blocks modifications to all keys by all
    /// processes in the registry hive that is being flushed until the flush operation completes.
    /// [`RegFlushKey`] should only be called explicitly when an application must guarantee that
    /// registry changes are persisted to disk immediately after modification. All modifications
    /// made to keys are visible to other processes without the need to flush them to disk.
    ///
    /// Alternatively, the registry has a 'lazy flush' mechanism that flushes registry
    /// modifications to disk at regular intervals of time. In addition to this regular flush
    /// operation, registry changes are also flushed to disk at system shutdown. Allowing the
    /// 'lazy flush' to flush registry changes is the most efficient way to manage registry writes
    /// to the registry store on disk.
    ///
    /// The [`RegFlushKey`] function returns only when all the data for the hive that contains the
    /// specified key has been written to the registry store on disk.
    ///
    /// The [`RegFlushKey`] function writes out the data for other keys in the hive that have been
    /// modified since the last lazy flush or system start.
    ///
    /// After [`RegFlushKey`] returns, use [`RegCloseKey`] to close the handle to the registry key.
    pub fn RegFlushKey(key: HKEY) -> LSTATUS;
}
