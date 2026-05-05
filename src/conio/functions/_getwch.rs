use crate::wint_t;

unsafe extern "C" {
    /// Gets a character from the console without echo.
    ///
    /// # Return Value
    /// Returns the character read. There's no error return.
    ///
    /// # Remarks
    /// The [`_getwch`] function read a single character from the console without echoing the
    /// character. To read a function key or arrow key, each function must be called twice. The
    /// first call returns 0 or 0xE0. The second call returns the key scan code.
    ///
    /// This function locks the calling thread and so is thread-safe. For the non-locking version,
    /// see [`_getwch_nolock`].
    ///
    /// By default, this function's global state is scoped to the application.
    pub fn _getwch() -> wint_t;
}
