// rustdoc imports
#[allow(unused_imports)]
use crate::MAKEINTRESOURCE;

/// Converts an integer value to a resource type compatible with the resource-management functions.
/// This macro is used in place of a string containing the name of the resource.
///
/// # Parameters
///  * `i` - The integer value to be converted.
///
/// # Remarks
/// The return value should be passed only to functions which explicitly indicate that they accept
/// [`MAKEINTRESOURCE`] as a parameter. For example, the resource management functions allow the
/// return value of [`MAKEINTRESOURCE`] to be passed as the `r#type` or `name` parameters.
#[macro_export]
macro_rules! MAKEINTRESOURCEW {
    ($i: expr) => {
        (($i as $crate::WORD) as $crate::ULONG_PTR) as $crate::LPWSTR
    };
}
