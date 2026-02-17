use crate::{BOOL, HANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateEvent, GetLastError};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Sets the specified event object to the signaled state.
    ///
    /// # Parameters
    ///  * `event` - A handle to the event object. The [`CreateEvent`] or [`OpenEvent`] function
    ///              returns this handle. The handle must have the `EVENT_MODIFY_STATE` access
    ///              right.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// The state of a manual-reset event object remains signaled until it is set explicitly to the
    /// nonsignaled state by the [`ResetEvent`] function. Any number of waiting threads, or threads
    /// that subsequently begin wait operations for the specified event object by calling one of
    /// the wait functions, can be released while the object's state is signaled.
    ///
    /// In contrast, the state of an auto-reset event object remains signaled until a single
    /// waiting thread is released, at which time the system automatically sets the state to
    /// nonsignaled. If no threads are waiting, the event object's state remains signaled.
    ///
    /// Setting an event that is already set has no effect.
    pub fn SetEvent(event: HANDLE) -> BOOL;
}
