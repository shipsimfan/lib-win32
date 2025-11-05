// rustdoc imports
#[allow(unused_imports)]
use crate::RAWINPUT;

/// Retrieves the location of the next structure in an array of [`RAWINPUT`] structures.
///
/// # Parameters
///  * `ptr` - A pointer to a structure in an array of [`RAWINPUT`] structures.
///
/// # Return Value
/// The return value is a pointer to the next structure in the array of [`RAWINPUT`] structures.
///
/// # Remarks
/// This macro is called repeatedly to traverse an array of [`RAWINPUT`] structures.
#[macro_export]
macro_rules! NEXTRAWINPUTBLOCK {
    ($ptr: expr) => {
        $crate::RAWINPUT_ALIGN!(
            unsafe { ($ptr as *mut $crate::PBYTE + (*ptr).header.size as _) } as $crate::ULONG_PTR
        ) as $crate::PRAWINPUT
    };
}
