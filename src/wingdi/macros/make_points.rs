// rustdoc imports
#[allow(unused_imports)]
use crate::POINTS;

/// The [`MAKEPOINTS`] macro converts a value that contains the x- and y-coordinates of a point
/// into a [`POINTS`] structure.
///
/// # Parameters
///  * `l` - The coordinates of a point. The x-coordinate is in the low-order word, and the
///          y-coordinate is in the high-order word.
///
/// # Return Value
/// The return value is a pointer to a [`POINTS`] structure.
#[macro_export]
macro_rules! MAKEPOINTS {
    ($l: expr) => {
        unsafe { *((&$l) as *const $crate::POINTS) }
    };
}
