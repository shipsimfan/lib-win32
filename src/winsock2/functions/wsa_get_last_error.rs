use std::ffi::c_int;

#[link(name = "Ws2_32")]
extern "system" {
    /// The [`WSAGetLastError`] function returns the error status for the last Windows Sockets
    /// operation that failed.
    ///
    /// # Return Value
    /// The return value indicates the error code for this thread's last Windows Sockets operation
    /// that failed.
    ///
    /// # Remarks
    /// The [`WSAGetLastError`] function returns the last error that occurred for the calling
    /// thread. When a particular Windows Sockets function indicates an error has occurred, this
    /// function should be called immediately to retrieve the extended error code for the failing
    /// function call. This extended error code can be different from the error code obtained from
    /// [`getsockopt`] when called with an `optname` parameter of [`SO_ERROR`], which is
    /// socket-specific since [`WSAGetLastError`] is for all thread-specific sockets.
    ///
    /// If a function call's return value indicates that error or other relevant data was returned
    /// in the error code, [`WSAGetLastError`] should be called immediately. This is necessary
    /// because some functions may reset the last extended error code to 0 if they succeed,
    /// overwriting the extended error code returned by a previously failed function. To
    /// specifically reset the extended error code, use the [`WSASetLastError`] function call with
    /// the `error` parameter set to zero. A [`getsockopt`] function when called with an `optname`
    /// parameter of [`SO_ERROR`] also resets the extended error code to zero.
    ///
    /// The [`WSAGetLastError`] function should not be used to check for an extended error value on
    /// receipt of an asynchronous message. In this case, the extended error value is passed in the
    /// `l_param` parameter of the message, and this can differ from the value returned by
    /// [`WSAGetLastError`].
    pub fn WSAGetLastError() -> c_int;
}
