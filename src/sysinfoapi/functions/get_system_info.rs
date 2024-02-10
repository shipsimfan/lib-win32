use crate::LPSYSTEM_INFO;

// rustdoc imports
#[allow(unused_imports)]
use crate::SYSTEM_INFO;

#[link(name = "Kernel32")]
extern "system" {
    /// Retrieves information about the current system.
    ///
    /// To retrieve accurate information for an application running on WOW64, call the
    /// [`GetNativeSystemInfo`] function.
    ///
    /// # Parameters
    ///  * `system_info` - A pointer to a [`SYSTEM_INFO`] structure that receives the information.
    pub fn GetSystemInfo(system_info: LPSYSTEM_INFO);
}
