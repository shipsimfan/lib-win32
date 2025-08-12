use crate::{
    c_size_t,
    strsafe::{STRSAFE_LPCWSTR, STRSAFE_LPWSTR},
    DWORD, HRESULT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    strsafe::{
        StringCchPrintf, StringCchPrintfEx, STRSAFE_E_INSUFFICIENT_BUFFER,
        STRSAFE_E_INVALID_PARAMETER, STRSAFE_FILL_BEHIND_NULL, STRSAFE_FILL_ON_FAILURE,
        STRSAFE_IGNORE_NULLS, STRSAFE_MAX_CCH, STRSAFE_NO_TRUNCATION, STRSAFE_NULL_ON_FAILURE,
        S_OK,
    },
    FAILED, SUCCEEDED,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

extern "C" {
    /// Writes formatted data to the specified string. The size of the destination buffer is
    /// provided to the function to ensure that it does not write past the end of this buffer.
    ///
    /// [`StringCchPrintfEx`] adds to the functionality of [`StringCchPrintf`] by returning a
    /// pointer to the end of the destination string as well as the number of characters left
    /// unused in that string. Flags may also be passed to the function for additional control.
    ///
    /// # Parameters
    ///  * `psz_dest` - The destination buffer, which receives the formatted, null-terminated
    ///                 string created from `psz_format` and its arguments.
    ///  * `ccd_dest` - The size of the destination buffer, in characters. This value must be
    ///                 sufficiently large to accommodate the final formatted string plus 1 to
    ///                 account for the terminating null character. The maximum number of  
    ///                 characters allowed is [`STRSAFE_MAX_CCH`].
    ///  * `ppsz_dest_end` - The address of a pointer to the end of `psz_dest`. If `ppsz_dest_end`
    ///                      is not [`null_mut`] and any data is copied into the destination
    ///                      buffer, this points to the terminating null character at the end of
    ///                      the string.
    ///  * `pcch_remaining` - The number of unused characters in `psz_dest`, including the
    ///                       terminating null character. If `pcch_remaining` is [`null_mut`], the
    ///                       count is not kept or returned.
    ///  * `flags` - One or more of the following values:
    ///    * [`STRSAFE_FILL_BEHIND_NULL`] - If the function succeeds, the low byte of `flags` (0)
    ///                                     is used to fill the uninitialized portion of `psz_dest`
    ///                                     following the terminating null character.
    ///    * [`STRSAFE_IGNORE_NULLS`] - Treat [`null`] string pointers like empty strings.
    ///    * [`STRSAFE_FILL_ON_FAILURE`] - If the function fails, the low byte of `flags` (0) is
    ///                                    used to fill the entire `psz_dest` buffer, and the
    ///                                    buffer is null-terminated. In the case of a
    ///                                    [`STRSAFE_E_INSUFFICIENT_BUFFER`] failure, any truncated
    ///                                    string returned is overwritten.
    ///    * [`STRSAFE_NULL_ON_FAILURE`] - If the function fails, `psz_dest` is set to an empty
    ///                                    string. In the case of a
    ///                                    [`STRSAFE_E_INSUFFICIENT_BUFFER`] failure, any truncated
    ///                                    string is overwritten.
    ///    * [`STRSAFE_NO_TRUNCATION`] - As in the case of [`STRSAFE_NULL_ON_FAILURE`], if the
    ///                                  function fails, `psz_dest` is set to an empty string. In
    ///                                  the case of a [`STRSAFE_E_INSUFFICIENT_BUFFER`] failure,
    ///                                  any truncated string is overwritten.
    ///  * `psz_format` - The format string. This string must be null-terminated.
    ///  * `...` - The arguments to be inserted into the `psz_format` string.
    ///
    /// # Return Value
    /// This function can return one of the following values. It is strongly recommended that you
    /// use the [`SUCCEEDED`] and [`FAILED`] macros to test the return value of this function.
    ///  * [`S_OK`] - There was sufficient space for the result to be copied to `psz_dest` without
    ///               truncation and the buffer is null-terminated.
    ///  * [`STRSAFE_E_INVALID_PARAMETER`] - The value in `cch_dest` is either 0 or larger than
    ///                                      [`STRSAFE_MAX_CCH`], or the destination buffer is
    ///                                      already full.
    ///  * [`STRSAFE_E_INSUFFICIENT_BUFFER`] - The copy operation failed due to insufficient buffer
    ///                                        space. Depending on the value of `flags`, the
    ///                                        destination buffer may contain a truncated,
    ///                                        null-terminated version of the intended result. In
    ///                                        situations where truncation is acceptable, this may
    ///                                        not necessarily be seen as a failure condition.
    ///
    /// Note that this function returns an [`HRESULT`] value, unlike the functions that it
    /// replaces.
    ///
    /// # Remarks
    /// Compared to the functions it replaces, [`StringCchPrintfEx`] provides additional processing
    /// for proper buffer handling in your code. Poor buffer handling is implicated in many
    /// security issues that involve buffer overruns. [`StringCchPrintfEx`] always null-terminates
    /// a nonzero-length destination buffer.
    ///
    /// Behavior is undefined if the strings pointed to by `psz_dest`, `psz_format`, or any
    /// argument strings overlap.
    ///
    /// Neither `psz_format` nor `psz_dest` should be [`null`] unless the [`STRSAFE_IGNORE_NULLS`]
    /// flag is specified, in which case both may be [`null`]. However, an error due to
    /// insufficient space may still be returned even though NULL values are ignored.
    pub fn StringCchPrintfExW(
        psz_dest: STRSAFE_LPWSTR,
        cch_dest: c_size_t,
        ppsz_dest_end: *mut STRSAFE_LPWSTR,
        pcch_remaining: *mut c_size_t,
        flags: DWORD,
        psz_format: STRSAFE_LPCWSTR,
        ...
    ) -> HRESULT;
}
