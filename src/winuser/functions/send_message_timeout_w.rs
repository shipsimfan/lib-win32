use crate::{HWND, LPARAM, LRESULT, PDWORD_PTR, UINT, WPARAM};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    GetLastError, GetMessage, SendMessageTimeout, SetLastError, ERROR_SUCCESS, HWND_BROADCAST,
    SMTO_ABORTIFHUNG, SMTO_BLOCK, SMTO_ERRORONEXIT, SMTO_NORMAL, SMTO_NOTIMEOUTIFNOTHUNG,
};

#[link(name = "User32")]
extern "system" {
    /// Sends the specified message to one or more windows.
    ///
    /// # Parameters
    ///  * `wnd` - A handle to the window whose window procedure will receive the message. If this
    ///            parameter is [`HWND_BROADCAST`], the message is sent to all top-level windows in
    ///            the system, including disabled or invisible unowned windows. The function does
    ///            not return until each window has timed out. Therefore, the total wait time can
    ///            be up to the value of `timeout` multiplied by the number of top-level windows.
    ///  * `msg` - The message to be sent.
    ///  * `w_param` - Any additional message-specific information.
    ///  * `l_param` - Any additional message-specific information.
    ///  * `flags` - The behavior of this function. This parameter can be one or more of the
    ///              following values:
    ///    * [`SMTO_ABORTIFHUNG`] - The function returns without waiting for the time-out period to
    ///                             elapse if the receiving thread appears to not respond or
    ///                             "hangs."
    ///    * [`SMTO_BLOCK`] - Prevents the calling thread from processing any other requests until
    ///                       the function returns.
    ///    * [`SMTO_NORMAL`] - The calling thread is not prevented from processing other requests
    ///                        while waiting for the function to return.
    ///    * [`SMTO_NOTIMEOUTIFNOTHUNG`] - The function does not enforce the time-out period as
    ///                                    long as the receiving thread is processing messages.
    ///    * [`SMTO_ERRORONEXIT`] - The function should return 0 if the receiving window is
    ///                             destroyed or its owning thread dies while the message is being
    ///                             processed.
    ///  * `timeout` - The duration of the time-out period, in milliseconds. If the message is a
    ///                broadcast message, each window can use the full time-out period. For
    ///                example, if you specify a five second time-out period and there are three
    ///                top-level windows that fail to process the message, you could have up to a
    ///                15 second delay.
    ///  * `result` - The result of the message processing. The value of this parameter depends on
    ///               the message that is specified.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is nonzero. [`SendMessageTimeout`] does not
    /// provide information about individual windows timing out if [`HWND_BROADCAST`] is used.
    ///
    /// If the function fails or times out, the return value is 0. Note that the function does not
    /// always call [`SetLastError`] on failure. If the reason for failure is important to you,
    /// call `SetLastError(ERROR_SUCCESS)` before calling [`SendMessageTimeout`]. If the function
    /// returns 0, and [`GetLastError`] returns [`ERROR_SUCCESS`], then treat it as a generic
    /// failure.
    ///
    /// # Remarks
    /// The function calls the window procedure for the specified window and, if the specified
    /// window belongs to a different thread, does not return until the window procedure has
    /// processed the message or the specified time-out period has elapsed. If the window receiving
    /// the message belongs to the same queue as the current thread, the window procedure is called
    /// directly — the time-out value is ignored.
    ///
    /// This function considers that a thread is not responding if it has not called [`GetMessage`]
    /// or a similar function within five seconds.
    ///
    /// The system only does marshalling for system messages (those in the range 0 to
    /// `WM_USER - 1`). To send other messages (those `>= WM_USER`) to another process, you must do
    /// custom marshalling.
    pub fn SendMessageTimeoutW(
        wnd: HWND,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
        flags: UINT,
        timeout: UINT,
        result: PDWORD_PTR,
    ) -> LRESULT;
}
