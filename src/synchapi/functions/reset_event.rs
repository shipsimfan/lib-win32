use crate::{BOOL, HANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateEvent, GetLastError, SetEvent};

#[link(name = "Kernel32")]
unsafe extern "system" {
    /// Sets the specified event object to the nonsignaled state.
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
    /// The state of an event object remains nonsignaled until it is explicitly set to signaled by
    /// the [`SetEvent`] or [`PulseEvent`] function. This nonsignaled state blocks the execution of
    /// any threads that have specified the event object in a call to one of the wait functions.
    ///
    /// The [`ResetEvent`] function is used primarily for manual-reset event objects, which must be
    /// set explicitly to the nonsignaled state. Auto-reset event objects automatically change from
    /// signaled to nonsignaled after a single waiting thread is released.
    ///
    /// Resetting an event that is already reset has no effect.
    pub fn ResetEvent(event: HANDLE) -> BOOL;
}
