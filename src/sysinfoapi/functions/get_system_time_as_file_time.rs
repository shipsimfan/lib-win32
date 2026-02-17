use crate::LPFILETIME;

// rustdoc imports
#[allow(unused_imports)]
use crate::FILETIME;

#[link(name = "Kernel32")]
unsafe extern "C" {
    /// Retrieves the current system date and time. The information is in Coordinated Universal
    /// Time (UTC) format.
    ///
    /// # Parameters
    ///  * `system_time_as_file_time` - A pointer to a [`FILETIME`] structure to receive the
    ///                                 current system date and time in UTC format.
    pub fn GetSystemTimeAsFileTime(system_time_as_file_time: LPFILETIME);
}
