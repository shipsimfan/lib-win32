use crate::{BOOL, LARGE_INTEGER};

// rustdoc imports
#[allow(unused_imports)]
use crate::GetLastError;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves the frequency of the performance counter. The frequency of the performance
    /// counter is fixed at system boot and is consistent across all processors. Therefore, the
    /// frequency need only be queried upon application initialization, and the result can be
    /// cached.
    ///
    /// # Parameters
    ///  * `frequency` - A pointer to a variable that receives the current performance-counter
    ///                  frequency, in counts per second. If the installed hardware doesn't support
    ///                  a high-resolution performance counter, this parameter can be zero (this
    ///                  will not occur on systems that run Windows XP or later).
    ///
    /// # Return Value
    /// If the installed hardware supports a high-resolution performance counter, the return value
    /// is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`]. On systems that run Windows XP or later, the function will always succeed
    /// and will thus never return zero.
    pub fn QueryPerformanceFrequency(frequency: *mut LARGE_INTEGER) -> BOOL;
}
