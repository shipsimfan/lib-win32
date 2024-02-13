use crate::{HKEY, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCreateKeyEx, RegOpenKeyEx, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM,
    KEY_CREATE_SUB_KEY, KEY_READ,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
extern "system" {
    /// Copies the specified registry key, along with its values and subkeys, to the specified
    /// destination key.
    ///
    /// # Parameters
    ///  * `key_src` - A handle to an open registry key. The key must have been opened with the
    ///                [`KEY_READ`] access right. This handle is returned by the [`RegCreateKeyEx`]
    ///                or [`RegOpenKeyEx`] function, or it can be one of the predefined keys.
    ///  * `sub_key` - The name of the key. This key must be a subkey of the key identified by the
    ///                `key_src` parameter. This parameter can also be [`null`].
    ///  * `key_dest` - A handle to the destination key. The calling process must have
    ///                 [`KEY_CREATE_SUB_KEY`] access to the key. This handle is returned by the
    ///                 [`RegCreateKeyEx`] or [`RegOpenKeyEx`] function, or it can be one of the
    ///                 predefined keys.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// This function also copies the security descriptor for the key.
    pub fn RegCopyTreeW(key_src: HKEY, sub_key: LPCWSTR, key_dest: HKEY) -> LSTATUS;
}
