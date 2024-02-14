use core::ffi::c_size_t;

extern "C" {
    /// Gets the length of a string, by using the current locale
    ///
    /// # Parameters
    ///  * `str` - Null-terminated string.
    ///
    /// # Return Value
    /// This function returns the number of characters in `str`, excluding the terminal null. No
    /// return value is reserved to indicate an error.
    ///
    /// # Remarks
    /// [`strlen`] interprets the string as a single-byte character string, so its return value is
    /// always equal to the number of bytes, even if the string contains multibyte characters.
    /// [`wcslen`] is a wide-character version of [`strlen`]; the argument of [`wcslen`] is a
    /// wide-character string and the count of characters is in wide (two-byte) characters.
    /// [`wcslen`] and [`strlen`] behave identically otherwise.
    pub fn wcslen(str: *const u16) -> c_size_t;
}
