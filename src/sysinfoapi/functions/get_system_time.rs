use crate::LPSYSTEMTIME;

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetLocalTime, SYSTEMTIME};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Kernel32")]
extern "C" {
    /// Retrieves the current system date and time in Coordinated Universal Time (UTC) format.
    ///
    /// To retrieve the current system date and time in local time, use the [`GetLocalTime`]
    /// function.
    ///
    /// # Parameters
    ///  * `system_time` - A pointer to a [`SYSTEMTIME`] structure to receive the current system
    ///                    date and time. The `system_time` parameter must not be [`null_mut`].
    ///                    Using [`null_mut`] will result in an access violation.
    ///
    /// # Remarks
    /// To set the current system date and time, use the [`SetSystemTime`] function.
    pub fn GetSystemTime(system_time: LPSYSTEMTIME);
}
