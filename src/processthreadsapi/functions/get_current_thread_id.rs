use crate::DWORD;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves the thread identifier of the calling thread.
    ///
    /// # Return Value
    /// The return value is the thread identifier of the calling thread.
    ///
    /// # Remarks
    /// Until the thread terminates, the thread identifier uniquely identifies the thread
    /// throughout the system.
    pub fn GetCurrentThreadId() -> DWORD;
}
