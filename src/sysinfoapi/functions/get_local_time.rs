use crate::LPSYSTEMTIME;

// rustdoc imports
#[allow(unused_imports)]
use crate::{GetSystemTime, SYSTEMTIME};

#[link(name = "Kernel32")]
unsafe extern "C" {
    /// Retrieves the current local date and time.
    ///
    /// To retrieve the current date and time in Coordinated Universal Time (UTC) format, use the
    /// [`GetSystemTime`] function.
    ///
    /// # Parameters
    ///  * `system_time` - A pointer to a [`SYSTEMTIME`] structure to receive the current local
    ///                    date and time.
    ///
    /// # Remarks
    /// To set the current local date and time, use the [`SetLocalTime`] function.
    pub fn GetLocalTime(system_time: LPSYSTEMTIME);
}
