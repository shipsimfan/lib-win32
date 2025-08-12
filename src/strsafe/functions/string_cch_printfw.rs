use crate::{
    c_size_t,
    strsafe::{STRSAFE_LPCWSTR, STRSAFE_LPWSTR},
    HRESULT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    strsafe::{
        StringCchPrintf, StringCchPrintfEx, STRSAFE_E_INSUFFICIENT_BUFFER,
        STRSAFE_E_INVALID_PARAMETER, STRSAFE_MAX_CCH, S_OK,
    },
    FAILED, SUCCEEDED,
};
#[allow(unused_imports)]
use std::ptr::null;

extern "C" {
    /// Writes formatted data to the specified string. The size of the destination buffer is
    /// provided to the function to ensure that it does not write past the end of this buffer.
    ///
    /// # Parameters
    ///  * `psz_dest` - The destination buffer, which receives the formatted, null-terminated
    ///                 string created from `psz_format` and its arguments.
    ///  * `cch_dest` - The size of the destination buffer, in characters. This value must be
    ///                 sufficiently large to accommodate the final formatted string plus 1 to
    ///                 account for the terminating null character. The maximum number of
    ///                 characters allowed is [`STRSAFE_MAX_CCH`].
    ///  * `psz_format` - The format string. This string must be null-terminated.
    ///  * `...` - The arguments to be inserted into the `psz_format` string.
    ///
    /// # Return Value
    /// This function can return one of the following values. It is strongly recommended that you
    /// use the [`SUCCEEDED`] and [`FAILED`] macros to test the return value of this function.
    ///  * [`S_OK`] - There was sufficient space for the result to be copied to `psz_dest` without
    ///               truncation, and the buffer is null-terminated.
    ///  * [`STRSAFE_E_INVALID_PARAMETER`] - The value in `cch_dest` is either 0 or larger than
    ///                                      [`STRSAFE_MAX_CCH`].
    ///  * [`STRSAFE_E_INSUFFICIENT_BUFFER`] - The copy operation failed due to insufficient buffer
    ///                                        space. The destination buffer contains a truncated,
    ///                                        null-terminated version of the intended result. In
    ///                                        situations where truncation is acceptable, this may
    ///                                        not necessarily be seen as a failure condition.
    ///
    /// Note that this function returns an [`HRESULT`] value, unlike the functions that it
    /// replaces.
    ///
    /// # Remarks
    /// Compared to the functions it replaces, [`StringCchPrintf`] provides additional processing
    /// for proper buffer handling in your code. Poor buffer handling is implicated in many
    /// security issues that involve buffer overruns. [`StringCchPrintf`] always null-terminates a
    /// nonzero-length destination buffer.
    ///
    /// Behavior is undefined if the strings pointed to by `psz_dest`, `psz_format`, or any
    /// argument strings overlap.
    ///
    /// Neither `psz_format` nor `psz_dest` should be [`null`]. See [`StringCchPrintfEx`] if you
    /// require the handling of null string pointer values.
    pub fn StringCchPrintfW(
        psz_dest: STRSAFE_LPWSTR,
        cch_dest: c_size_t,
        psz_format: STRSAFE_LPCWSTR,
        ...
    ) -> HRESULT;
}
