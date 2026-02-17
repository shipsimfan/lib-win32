use crate::{BOOL, HANDLE, LPCWSTR, LPSECURITY_ATTRIBUTES};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CloseHandle, CreateWaitableTimer, GetLastError, ERROR_ALREADY_EXISTS, ERROR_INVALID_HANDLE,
    SECURITY_ATTRIBUTES, TIMER_ALL_ACCESS, TRUE,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Creates or opens a waitable timer object.
    ///
    /// To specify an access mask for the object, use the [`CreateWaitableTimerEx`] function.
    ///
    /// # Parameters
    ///  * `timer_attributes` - A pointer to a [`SECURITY_ATTRIBUTES`] structure that specifies a
    ///                         security descriptor for the new timer object and determines whether
    ///                         child processes can inherit the returned handle. If
    ///                         `timer_attributes` is [`null_mut`], the timer object gets a default
    ///                         security descriptor and the handle cannot be inherited. The ACLs in
    ///                         the default security descriptor for a timer come from the primary
    ///                         or impersonation token of the creator.
    ///  * `manual_reset` - If this parameter is [`TRUE`], the timer is a manual-reset notification
    ///                     timer. Otherwise, the timer is a synchronization timer.
    ///  * `timer_name` - The name of the timer object. The name is limited to [`MAX_PATH`]
    ///                   characters. Name comparison is case sensitive. If `timer_name` is
    ///                   [`null`], the timer object is created without a name. If `timer_name`
    ///                   matches the name of an existing event, semaphore, mutex, job, or
    ///                   file-mapping object, the function fails and [`GetLastError`] returns
    ///                   [`ERROR_INVALID_HANDLE`]. This occurs because these objects share the
    ///                   same namespace. The name can have a "Global" or "Local" prefix to
    ///                   explicitly create the object in the global or session namespace. The
    ///                   remainder of the name can contain any character except the backslash
    ///                   character (\). Fast user switching is implemented using Terminal Services
    ///                   sessions. Kernel object names must follow the guidelines outlined for
    ///                   Terminal Services so that applications can support multiple users. The
    ///                   object can be created in a private namespace.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the timer object. If the named
    /// timer object exists before the function call, the function returns a handle to the existing
    /// object and [`GetLastError`] returns [`ERROR_ALREADY_EXISTS`].
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// The handle returned by [`CreateWaitableTimer`] is created with the [`TIMER_ALL_ACCESS`]
    /// access right; it can be used in any function that requires a handle to a timer object,
    /// provided that the caller has been granted access. If a timer is created from a service or
    /// thread that is impersonating a different user, you can either apply a security descriptor
    /// to the timer when you create it, or change the default security descriptor for the creating
    /// process by changing its default DACL.
    ///
    /// Any thread of the calling process can specify the timer object handle in a call to one of
    /// the wait functions.
    ///
    /// Multiple processes can have handles to the same timer object, enabling use of the object
    /// for interprocess synchronization.
    ///
    /// A process created by the [`CreateProcess`] function can inherit a handle to a timer object
    /// if the `timer_attributes` parameter of [`CreateWaitableTimer`] enables inheritance.
    ///  - A process can specify the timer object handle in a call to the [`DuplicateHandle`]
    ///    function. The resulting handle can be used by another process.
    ///  - A process can specify the name of a timer object in a call to the [`OpenWaitableTimer`]
    ///    or [`CreateWaitableTimer`] function.
    ///
    /// Use the [`CloseHandle`] function to close the handle. The system closes the handle
    /// automatically when the process terminates. The timer object is destroyed when its last
    /// handle has been closed.
    ///
    /// To associate a timer with a window, use the [`SetTimer`] function.
    pub fn CreateWaitableTimerW(
        timer_attributes: LPSECURITY_ATTRIBUTES,
        manual_reset: BOOL,
        timer_name: LPCWSTR,
    ) -> HANDLE;
}
