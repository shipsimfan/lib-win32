// rustdoc imports
#[allow(unused_imports)]
use crate::LPTSTR;

/// Converts the specified atom into a string, so it can be passed to functions which accept either
/// atoms or strings.
///
/// # Parameters
///  * `i` - The numeric value to be made into an integer atom. This parameter can be either an
///          integer atom or a string atom.
///
/// # Remarks
/// Although the return value of the [`MAKEINTATOM`] macro is cast as an [`LPTSTR`] value, it
/// cannot be used as a string pointer except when it is passed to atom-management functions that
/// require an [`LPTSTR`] argument.
#[macro_export]
macro_rules! MAKEINTATOM {
    ($i: expr) => {
        $i as $crate::WORD as $crate::ULONG_PTR as $crate::LPTSTR
    };
}
