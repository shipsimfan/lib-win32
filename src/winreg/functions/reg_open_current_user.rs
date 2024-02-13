use crate::{LSTATUS, PHKEY, REGSAM};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    FormatMessage, RegCloseKey, ERROR_SUCCESS, FORMAT_MESSAGE_FROM_SYSTEM, HKEY_CURRENT_USER,
    HKEY_USERS,
};

#[link(name = "Advapi32")]
extern "system" {
    /// Retrieves a handle to the [`HKEY_CURRENT_USER`] key for the user the current thread is
    /// impersonating.
    ///
    /// # Parameters
    ///  * `desired` - A mask that specifies the desired access rights to the key. The function
    ///                fails if the security descriptor of the key does not permit the requested
    ///                access for the calling process.
    ///  * `result` - A pointer to a variable that receives a handle to the opened key. When you no
    ///               longer need the returned handle, call the [`RegCloseKey`] function to close
    ///               it.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`ERROR_SUCCESS`].
    ///
    /// If the function fails, the return value is a nonzero error code. You can use the
    /// [`FormatMessage`] function with the [`FORMAT_MESSAGE_FROM_SYSTEM`] flag to get a generic
    /// description of the error.
    ///
    /// # Remarks
    /// The [`HKEY_CURRENT_USER`] key maps to the root of the current user's branch in the
    /// [`HKEY_USERS`] key. It is cached for all threads in a process. Therefore, this value does
    /// not change when another user's profile is loaded. [`RegOpenCurrentUser`] uses the thread's
    /// token to access the appropriate key, or the default if the profile is not loaded.
    pub fn RegOpenCurrentUser(desired: REGSAM, result: PHKEY) -> LSTATUS;
}
