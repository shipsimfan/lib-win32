use crate::{BOOL, HANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{CreateWaitableTimer, GetLastError, SetWaitableTimer, TIMER_MODIFY_STATE};

#[link(name = "Kernel32")]
extern "system" {
    /// Sets the specified waitable timer to the inactive state.
    ///
    /// # Parameters
    ///  * `timer` - A handle to the timer object. The [`CreateWaitableTimer`] or
    ///              [`OpenWaitableTimer`] function returns this handle. The handle must have the
    ///              [`TIMER_MODIFY_STATE`] access right.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// The [`CancelWaitableTimer`] function does not change the signaled state of the timer. It
    /// stops the timer before it can be set to the signaled state and cancels outstanding APCs.
    /// Therefore, threads performing a wait operation on the timer remain waiting until they time
    /// out or the timer is reactivated and its state is set to signaled. If the timer is already
    /// in the signaled state, it remains in that state.
    ///
    /// To reactivate the timer, call the [`SetWaitableTimer`] function.
    pub fn CancelWaitableTimer(timer: HANDLE) -> BOOL;
}
