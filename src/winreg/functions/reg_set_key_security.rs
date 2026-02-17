use crate::{HKEY, LSTATUS, PSECURITY_DESCRIPTOR, SECURITY_INFORMATION};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCloseKey, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, SECURITY_DESCRIPTOR,
};

#[link(name = "Advapi32")]
unsafe extern "system" {
    /// The [`RegSetKeySecurity`] function sets the security of an open registry key.
    ///
    /// # Parameters
    ///  * `key` - A handle to an open key for which the security descriptor is set.
    ///  * `security_information` - A set of bit flags that indicate the type of security
    ///                             information to set. This parameter can be a combination of the
    ///                             [`SECURITY_INFORMATION`] bit flags.
    ///  * `security_descriptor` - A pointer to a [`SECURITY_DESCRIPTOR`] structure that specifies
    ///                            the security attributes to set for the specified key.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// If `key` is one of the predefined keys, use the [`RegCloseKey`] function to close the
    /// predefined key to ensure that the new security information is in effect the next time the
    /// predefined key is referenced.
    pub fn RegSetKeySecurity(
        key: HKEY,
        security_information: SECURITY_INFORMATION,
        security_descriptor: PSECURITY_DESCRIPTOR,
    ) -> LSTATUS;
}
