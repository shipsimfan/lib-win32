use crate::{BOOL, HANDLE, LPCWSTR, LPSECURITY_ATTRIBUTES};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CloseHandle, CreateEvent, GetLastError, ERROR_ALREADY_EXISTS, ERROR_INVALID_HANDLE, FALSE,
    SECURITY_ATTRIBUTES, TRUE,
};
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Creates or opens a named or unnamed event object.
    ///
    /// To specify an access mask for the object, use the [`CreateEventEx`] function.
    ///
    /// # Parameters
    ///  * `event_attributes` - A pointer to a [`SECURITY_ATTRIBUTES`] structure. If this parameter
    ///                         is [`null_mut`], the handle cannot be inherited by child processes.
    ///                         The `security_descriptor` member of the structure specifies a
    ///                         security descriptor for the new event. If `event_attributes` is
    ///                         [`null_mut`], the event gets a default security descriptor. The
    ///                         ACLs in the default security descriptor for an event come from the
    ///                         primary or impersonation token of the creator.
    ///  * `manual_reset` - If this parameter is [`TRUE`], the function creates a manual-reset
    ///                     event object, which requires the use of the [`ResetEvent`] function to
    ///                     set the event state to nonsignaled. If this parameter is [`FALSE`], the
    ///                     function creates an auto-reset event object, and system automatically
    ///                     resets the event state to nonsignaled after a single waiting thread has
    ///                     been released.
    ///  * `initial_state` - If this parameter is [`TRUE`], the initial state of the event object
    ///                      is signaled; otherwise, it is nonsignaled.
    ///  * `name` - The name of the event object. The name is limited to [`MAX_PATH`] characters.
    ///             Name comparison is case sensitive. If `name` matches the name of an existing
    ///             named event object, this function requests the [`EVENT_ALL_ACCESS`] access
    ///             right. In this case, the `manual_reset` and `initial_state` parameters are
    ///             ignored because they have already been set by the creating process. If the
    ///             `event_attributes` parameter is not [`null_mut`], it determines whether the
    ///             handle can be inherited, but its security-descriptor member is ignored. If
    ///             `name` is [`null`], the event object is created without a name. If `name`
    ///             matches the name of another kind of object in the same namespace (such as an
    ///             existing semaphore, mutex, waitable timer, job, or file-mapping object), the
    ///             function fails and the [`GetLastError`] function returns
    ///             [`ERROR_INVALID_HANDLE`]. This occurs because these objects share the same
    ///             namespace. The name can have a "Global" or "Local" prefix to explicitly create
    ///             the object in the global or session namespace. The remainder of the name can
    ///             contain any character except the backslash character (\). Fast user switching
    ///             is implemented using Terminal Services sessions. Kernel object names must
    ///             follow the guidelines outlined for Terminal Services so that applications can
    ///             support multiple users. The object can be created in a private namespace.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is a handle to the event object. If the named
    /// event object existed before the function call, the function returns a handle to the
    /// existing object and [`GetLastError`] returns [`ERROR_ALREADY_EXISTS`].
    ///
    /// If the function fails, the return value is [`null_mut`]. To get extended error information,
    /// call [`GetLastError`].
    ///
    /// # Remarks
    /// The handle returned by [`CreateEvent`] has the [`EVENT_ALL_ACCESS`] access right; it can be
    /// used in any function that requires a handle to an event object, provided that the caller
    /// has been granted access. If an event is created from a service or a thread that is
    /// impersonating a different user, you can either apply a security descriptor to the event
    /// when you create it, or change the default security descriptor for the creating process by
    /// changing its default DACL.
    ///
    /// Any thread of the calling process can specify the event-object handle in a call to one of
    /// the wait functions. The single-object wait functions return when the state of the specified
    /// object is signaled. The multiple-object wait functions can be instructed to return either
    /// when any one or when all of the specified objects are signaled. When a wait function
    /// returns, the waiting thread is released to continue its execution.
    ///
    /// The initial state of the event object is specified by the `initial_state` parameter. Use
    /// the [`SetEvent`] function to set the state of an event object to signaled. Use the
    /// [`ResetEvent`] function to reset the state of an event object to nonsignaled.
    ///
    /// When the state of a manual-reset event object is signaled, it remains signaled until it is
    /// explicitly reset to nonsignaled by the [`ResetEvent`] function. Any number of waiting
    /// threads, or threads that subsequently begin wait operations for the specified event object,
    /// can be released while the object's state is signaled.
    ///
    /// When the state of an auto-reset event object is signaled, it remains signaled until a
    /// single waiting thread is released; the system then automatically resets the state to
    /// nonsignaled. If no threads are waiting, the event object's state remains signaled.
    ///
    /// Multiple processes can have handles of the same event object, enabling use of the object
    /// for interprocess synchronization. The following object-sharing mechanisms are available:
    ///  - A child process created by the [`CreateProcess`] function can inherit a handle to an
    ///    event object if the `event_attributes` parameter of [`CreateEvent`] enabled inheritance.
    ///  - A process can specify the event-object handle in a call to the [`DuplicateHandle`]
    ///    function to create a duplicate handle that can be used by another process.
    ///  - A process can specify the name of an event object in a call to the [`OpenEvent`] or
    ///    [`CreateEvent`] function.
    ///
    /// Use the [`CloseHandle`] function to close the handle. The system closes the handle
    /// automatically when the process terminates. The event object is destroyed when its last
    /// handle has been closed.
    pub fn CreateEventW(
        event_attributes: LPSECURITY_ATTRIBUTES,
        manual_reset: BOOL,
        initial_state: BOOL,
        name: LPCWSTR,
    ) -> HANDLE;
}
