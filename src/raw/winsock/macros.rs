#[allow(unused_imports)]
use crate::raw::Word;

/// # MAKEWORD macro
///
/// Creates a [`Word`] value by concatenating the specified values.
///
/// ## Parameters
///  - low - The low-order byte of the new value.
///  - high - The high-order byte of the new value.
///
/// ## Return Value
/// Type: [`Word`]
///
/// The return value is a [`Word`] value.
#[macro_export]
macro_rules! MAKEWORD {
    ($low: expr, $high: expr) => {
        ((($low as u32) & 0xFF) as Word) | (((($high as u32) & 0xFF) << 8) as Word)
    };
}
