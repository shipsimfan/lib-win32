use crate::{DWORD, LPVOID};

// rustdoc imports
#[allow(unused_imports)]
use crate::SetWaitableTimer;

/// An application-defined timer completion routine. Specify this address when calling the
/// [`SetWaitableTimer`] function. The [`PTIMERAPCROUTINE`] type defines a pointer to this callback
/// function.
pub type PTIMERAPCROUTINE = extern "system" fn(
    arg_to_completion_routine: LPVOID,
    timer_low_value: DWORD,
    timer_high_value: DWORD,
);
