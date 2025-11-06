// rustdoc imports
#[allow(unused_imports)]
use crate::{RIM_INPUT, RIM_INPUTSINK, WM_INPUT};

/// Retrieves the input code from `w_param` in [`WM_INPUT`] message.
///
/// # Parameters
///  * `w_param` - `w_param` from [`WM_INPUT`] message.
///
/// # Return Value
/// Input code value. Can be one of the following:
///  * [`RIM_INPUT`] - Input occurred while the application was in the foreground.
///  * [`RIM_INPUTSINK`] - Input occurred while the application was not in the foreground.
#[macro_export]
macro_rules! GET_RAWINPUT_CODE_WPARAM {
    ($w_param: expr) => {
        $w_param & 0xFF
    };
}
