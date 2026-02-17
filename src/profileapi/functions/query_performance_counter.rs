use crate::{BOOL, LARGE_INTEGER};

// rustdoc imports
#[allow(unused_imports)]
use crate::GetLastError;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves the current value of the performance counter, which is a high resolution (<1us)
    /// time stamp that can be used for time-interval measurements.
    ///
    /// # Parameters
    ///  * `performance_count` - A pointer to a variable that receives the current
    ///                          performance-counter value, in counts.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`]. On systems that run Windows XP or later, the function will always succeed
    /// and will thus never return zero.
    pub fn QueryPerformanceCounter(performance_counter: *mut LARGE_INTEGER) -> BOOL;
}
