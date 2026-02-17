use crate::{DWORD, HKEY, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, SHCopyKey, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CURRENT_USER,
};

#[link(name = "Shlwapi")]
unsafe extern "system" {
    /// Recursively copies the subkeys and values of the source subkey to the destination key.
    /// [`SHCopyKey`] does not copy the security attributes of the keys.
    ///
    /// # Parameters
    ///  * `src` - A handle to the source key (for example, [`HKEY_CURRENT_USER`]).
    ///  * `src_sub_key` - The subkey whose subkeys and values are to be copied.
    ///  * `dest` - The destination key.
    ///  * `reserved` - Reserved. Must be 0.
    ///
    /// # Return Value
    /// Returns [`ERROR_SUCCESS`] if successful, or one of the nonzero error codes. Use
    /// [`FormatMessage`] with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to retrieve a generic
    /// description of the error.
    pub fn SHCopyKeyW(src: HKEY, src_sub_key: LPCWSTR, dest: HKEY, reserved: DWORD) -> LSTATUS;
}
