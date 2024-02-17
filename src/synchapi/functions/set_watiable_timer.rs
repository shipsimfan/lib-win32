use crate::{BOOL, HANDLE, LARGE_INTEGER, LONG, LPVOID, PTIMERAPCROUTINE};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    CancelWaitableTimer, CreateWaitableTimer, GetLastError, ERROR_NOT_SUPPORTED, FILETIME,
    TIMER_MODIFY_STATE, TRUE,
};

#[link(name = "Kernel32")]
extern "system" {
    /// Activates the specified waitable timer. When the due time arrives, the timer is signaled
    /// and the thread that set the timer calls the optional completion routine.
    ///
    /// # Parameters
    ///  * `timer` - A handle to the timer object. The [`CreateWaitableTimer`] or
    ///              [`OpenWaitableTimer`] function returns this handle. The handle must have the
    ///              [`TIMER_MODIFY_STATE`] access right.
    ///  * `due_time` - The time after which the state of the timer is to be set to signaled, in
    ///                 100 nanosecond intervals. Use the format described by the [`FILETIME`]
    ///                 structure. Positive values indicate absolute time. Be sure to use a
    ///                 UTC-based absolute time, as the system uses UTC-based time internally.
    ///                 Negative values indicate relative time. The actual timer accuracy depends
    ///                 on the capability of your hardware.
    ///  * `period` - The period of the timer, in milliseconds. If `period` is zero, the timer is
    ///               signaled once. If `period` is greater than zero, the timer is periodic. A
    ///               periodic timer automatically reactivates each time the period elapses, until
    ///               the timer is canceled using the [`CancelWaitableTimer`] function or reset
    ///               using [`SetWaitableTimer`]. If `period` is less than zero, the function
    ///               fails.
    ///  * `completion_routine` - A pointer to an optional completion routine. The completion
    ///                           routine is application-defined function of type
    ///                           [`PTIMERAPCROUTINE`] to be executed when the timer is signaled.
    ///  * `arg_to_completion_routine` - A pointer to a structure that is passed to the completion
    ///                                  routine.
    ///  * `resume` - If this parameter is [`TRUE`], restores a system in suspended power
    ///               conservation mode when the timer state is set to signaled. Otherwise, the
    ///               system is not restored. If the system does not support a restore, the call
    ///               succeeds, but [`GetLastError`] returns [`ERROR_NOT_SUPPORTED`].
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero.
    ///
    /// If the function fails, the return value is zero. To get extended error information, call
    /// [`GetLastError`].
    ///
    /// # Remarks
    /// Timers are initially inactive. To activate a timer, call [`SetWaitableTimer`]. If the timer
    /// is already active when you call [`SetWaitableTimer`], the timer is stopped, then it is
    /// reactivated. Stopping the timer in this manner does not set the timer state to signaled, so
    /// threads blocked in a wait operation on the timer remain blocked. However, it does cancel
    /// any pending completion routines.
    ///
    /// When the specified due time arrives, the timer becomes inactive, and the optional APC is
    /// queued to the thread that set the timer if there is no outstanding APC already queued. The
    /// state of the timer is set to signaled, the timer is reactivated using the specified period,
    /// and the thread that set the timer calls the completion routine when it enters an alertable
    /// wait state. Note that APCs do not work as well as other signaling mechanisms for thread
    /// pool threads because the system controls the lifetime of thread pool threads, so it is
    /// possible for a thread to be terminated before the notification is delivered. Instead of
    /// using the `completion_routine` parameter or another APC-based signaling mechanism, use a
    /// waitable object such as a timer created with [`CreateThreadpoolTimer`]. For I/O, use an I/O
    /// completion object created with [`CreateThreadpoolIo`] or an `event`-based [`OVERLAPPED`]
    /// structure where the event can be passed to the [`SetThreadpoolWait`] function.
    ///
    /// If the thread that set the timer terminates and there is an associated completion routine,
    /// the timer is canceled. However, the state of the timer remains unchanged. If there is no
    /// completion routine, then terminating the thread has no effect on the timer.
    ///
    /// When a manual-reset timer is set to the signaled state, it remains in this state until
    /// [`SetWaitableTimer`] is called to reset the timer. As a result, a periodic manual-reset
    /// timer is set to the signaled state when the initial due time arrives and remains signaled
    /// until it is reset. When a synchronization timer is set to the signaled state, it remains in
    /// this state until a thread completes a wait operation on the timer object.
    ///
    /// If the system time is adjusted, the due time of any outstanding absolute timers is
    /// adjusted.
    ///
    /// To use a timer to schedule an event for a window, use the [`SetTimer`] function.
    ///
    /// APIs that deal with timers use various different hardware clocks. These clocks may have
    /// resolutions significantly different from what you expect: some may be measured in
    /// milliseconds (for those that use an RTC-based timer chip), to those measured in nanoseconds
    /// (for those that use ACPI or TSC counters). You can change the resolution of your API with a
    /// call to the [`timeBeginPeriod`] and [`timeEndPeriod`] functions. How precise you can change
    /// the resolution depends on which hardware clock the particular API uses.
    pub fn SetWaitableTimer(
        timer: HANDLE,
        due_time: *const LARGE_INTEGER,
        period: LONG,
        completion_routine: PTIMERAPCROUTINE,
        arge_to_completion_routine: LPVOID,
        resume: BOOL,
    ) -> BOOL;
}
