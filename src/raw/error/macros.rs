// rustdoc imports
#[allow(unused_imports)]
use crate::raw::HResult;

/// # FAILED macro (winerror.h)
///
/// Provides a generic test for failure on any status value.
///
/// ## Parameters
/// `hr`\
/// The status code. This value can be an [`HResult`] or an [`SCode`]. A
/// negative number indicates failure.
///
/// ## Return Value
/// `true` if `hr` represents a failed result, `false` otherwise.
#[macro_export]
macro_rules! FAILED {
    ($hr: expr) => {
        ($hr as $crate::raw::HResult) < 0
    };
}

/// # SUCCEEDED macro (winerror.h)
///
/// Provides a generic test for success on any status value.
///
/// ## Parameters
/// `hr`\
/// The status code. This value can be an [`HResult`] or an [`SCode`]. A
/// non-negative number indicates success.
///
/// ## Return Value
/// `true` if `hr` represents a succeeded result, `false` otherwise.
#[macro_export]
macro_rules! SUCCEEDED {
    ($hr: expr) => {
        ($hr as $crate::raw::HResult) >= 0
    };
}

/// The [`HResult`] From Win32 Error Code Macro converts a Win32 error code to
/// an [`HResult`] using the pattern 0x8007XXXX, where XXXX is the first two
/// bytes of the Win32 hex value 0x0000XXXX.
#[macro_export]
macro_rules! HRESULT_FROM_WIN32 {
    ($x: expr) => {
        if ($x as $crate::raw::HResult) <= 0 {
            $x as $crate::raw::HResult
        } else {
            (($x as $crate::raw::HResult) & 0x0000FFFF) | (0x0007 << 16) | -2147483648
        }
    };
}
