use crate::{HKEY, LPCWSTR, LSTATUS};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCreateKeyEx, RegDeleteKeyEx, RegDeleteTree, RegOpenKeyEx, SHDeleteKey,
    DELETE, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CLASSES_ROOT, HKEY_CURRENT_CONFIG,
    HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, HKEY_USERS,
};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "Advapi32")]
extern "system" {
    /// Deletes a subkey and its values. Note that key names are not case sensitive.
    ///
    /// 64-bit Windows: On WOW64, 32-bit applications view a registry tree that is separate from
    /// the registry tree that 64-bit applications view. To enable an application to delete an
    /// entry in the alternate registry view, use the [`RegDeleteKeyEx`] function.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open registry key. The access rights of this key do not affect
    ///            the delete operation. This handle is returned by the [`RegCreateKeyEx`] or
    ///            [`RegOpenKeyEx`] function, or it can be one of the following predefined keys:
    ///    * [`HKEY_CLASSES_ROOT`]
    ///    * [`HKEY_CURRENT_CONFIG`]
    ///    * [`HKEY_CURRENT_USER`]
    ///    * [`HKEY_LOCAL_MACHINE`]
    ///    * [`HKEY_USERS`]
    ///  * `sub_key` - The name of the key to be deleted. It must be a subkey of the key that `key`
    ///                identifies, but it cannot have subkeys. This parameter cannot be [`null`].
    ///                The function opens the subkey with the [`DELETE`] access right. Key names
    ///                are not case sensitive.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// A deleted key is not removed until the last handle to it is closed.
    ///
    /// The subkey to be deleted must not have subkeys. To delete a key and all its subkeys, you
    /// need to enumerate the subkeys and delete them individually. To delete keys recursively, use
    /// the [`RegDeleteTree`] or [`SHDeleteKey`] function.
    pub fn RegDeleteKeyW(key: HKEY, sub_key: LPCWSTR) -> LSTATUS;
}
