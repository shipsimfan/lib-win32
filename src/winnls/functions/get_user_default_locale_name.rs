use crate::LPWSTR;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLastError, ERROR_INSUFFICIENT_BUFFER, LOCALE_NAME_MAX_LENGTH};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves the user default locale name.
    ///
    /// # Parameters
    ///  * `locale_name` - Pointer to a buffer in which this function retrieves the locale name.
    ///  * `cch_locale_name` - Size, in characters, of the buffer indicated by `locale_name`. The
    ///                        maximum possible length of a locale name, including a terminating
    ///                        null character, is [`LOCALE_NAME_MAX_LENGTH`]. This is the
    ///                        recommended size to supply in this parameter.
    ///
    /// # Return Value
    /// Returns the size of the buffer containing the locale name, including the terminating null
    /// character, if successful.
    ///
    /// The function returns 0 if it does not succeed. To get extended error information, the
    /// application can call [`GetLastError`], which can return one of the following error codes:
    ///  * [`ERROR_INSUFFICIENT_BUFFER`] - A supplied buffer size was not large enough, or it was
    ///                                    incorrectly set to [`null_mut`].
    ///
    /// # Remarks
    /// This function can retrieve data from custom locales. Data is not guaranteed to be the same
    /// from computer to computer or between runs of an application.
    pub fn GetUserDefaultLocaleName(locale_name: LPWSTR, cch_locale_name: c_int) -> c_int;
}
