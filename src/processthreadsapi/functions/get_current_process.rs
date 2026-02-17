use crate::HANDLE;

// rustdoc imports
#[allow(unused_imports)]
use crate::CloseHandle;

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Retrieves a pseudo handle for the current process.
    ///
    /// # Return Value
    /// The return value is a pseudo handle to the current process.
    ///
    /// # Remarks
    /// A pseudo handle is a special constant, currently `-1 as HANDLE`, that is interpreted as the
    /// current process handle. For compatibility with future operating systems, it is best to call
    /// [`GetCurrentProcess`] instead of hard-coding this constant value. The calling process can
    /// use a pseudo handle to specify its own process whenever a process handle is required.
    /// Pseudo handles are not inherited by child processes.
    ///
    /// This handle has the [`PROCESS_ALL_ACCESS`] access right to the process object.
    ///
    /// Windows Server 2003 and Windows XP: This handle has the maximum access allowed by the
    /// security descriptor of the process to the primary token of the process.
    ///
    /// A process can create a "real" handle to itself that is valid in the context of other
    /// processes, or that can be inherited by other processes, by specifying the pseudo handle as
    /// the source handle in a call to the [`DuplicateHandle`] function. A process can also use the
    /// [`OpenProcess`] function to open a real handle to itself.
    ///
    /// The pseudo handle need not be closed when it is no longer needed. Calling the
    /// [`CloseHandle`] function with a pseudo handle has no effect. If the pseudo handle is
    /// duplicated by [`DuplicateHandle`], the duplicate handle must be closed.
    pub fn GetCurrentProcess() -> HANDLE;
}
