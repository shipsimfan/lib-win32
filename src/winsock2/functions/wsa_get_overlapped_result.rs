use crate::{
    winsock2::{types::LPWSAOVERLAPPED, SOCKET},
    BOOL, LPDWORD,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    winsock2::{WSAGetLastError, WSARecv, WSAOVERLAPPED},
    FALSE, TRUE,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "Ws2_32")]
unsafe extern "system" {
    /// The [`WSAGetOverlappedResult`] function retrieves the results of an overlapped operation on
    /// the specified socket.
    ///
    /// # Parameters
    ///  * `s` - A descriptor identifying the socket. This is the same socket that was specified
    ///          when the overlapped operation was started by a call to any of the Winsock
    ///          functions that supports overlapped operations. These functions include
    ///          [`AcceptEx`], [`ConnectEx`], [`DisconnectEx`], [`TransmitFile`],
    ///          [`TransmitPackets`], [`WSARecv`], [`WSARecvFrom`], [`LPFN_WSARECVMSG`]
    ///          ([`WSARecvMsg`]), [`WSASend`], [`WSASendMsg`], [`WSASendTo`], and [`WSAIoctl`].
    ///  * `overlapped` - A pointer to a [`WSAOVERLAPPED`] structure that was specified when the
    ///                   overlapped operation was started. This parameter must not be a
    ///                   [`null_mut`] pointer.
    ///  * `cb_transfer` - A pointer to a 32-bit variable that receives the number of bytes that
    ///                    were actually transferred by a send or receive operation, or by the
    ///                    [`WSAIoctl`] function. This parameter must not be a [`null_mut`]
    ///                    pointer.
    ///  * `wait` - A flag that specifies whether the function should wait for the pending
    ///             overlapped operation to complete. If [`TRUE`], the function does not return
    ///             until the operation has been completed. If [`FALSE`] and the operation is still
    ///             pending, the function returns [`FALSE`] and the [`WSAGetLastError`] function
    ///             returns [`WSA_IO_INCOMPLETE`]. The `wait` parameter may be set to [`TRUE`] only
    ///             if the overlapped operation selected the event-based completion notification.
    ///  * `flags` - A pointer to a 32-bit variable that will receive one or more flags that
    ///              supplement the completion status. If the overlapped operation was initiated
    ///              through [`WSARecv`] or [`WSARecvFrom`], this parameter will contain the
    ///              results value for `flags` parameter. This parameter must not be a [`null_mut`]
    ///              pointer.
    ///
    /// # Return Value
    /// If [`WSAGetOverlappedResult`] succeeds, the return value is [`TRUE`]. This means that the
    /// overlapped operation has completed successfully and that the value pointed to by
    /// `cb_transfer` has been updated.
    ///
    /// If [`WSAGetOverlappedResult`] returns [`FALSE`], this means that either the overlapped
    /// operation has not completed, the overlapped operation completed but with errors, or the
    /// overlapped operation's completion status could not be determined due to errors in one or
    /// more parameters to [`WSAGetOverlappedResult`]. On failure, the value pointed to by
    /// `cb_transfer` will not be updated. Use [`WSAGetLastError`] to determine the cause of the
    /// failure (either by the [`WSAGetOverlappedResult`] function or by the associated overlapped
    /// operation).
    ///
    /// # Remarks
    /// The [`WSAGetOverlappedResult`] function reports the results of the overlapped operation
    /// specified in the `overlapped` parameter for the socket specified in the s parameter. The
    /// [`WSAGetOverlappedResult`] function is passed the socket descriptor and the
    /// [`WSAOVERLAPPED`] structure that was specified when the overlapped function was called. A
    /// pending operation is indicated when the function that started the operation returns
    /// [`FALSE`] and the [`WSAGetLastError`] function returns [`WSA_IO_PENDING`]. When an I/O
    /// operation such as [`WSARecv`] is pending, the function that started the operation resets
    /// the `event` member of the [`WSAOVERLAPPED`] structure to the nonsignaled state. Then, when
    /// the pending operation has completed, the system sets the event object to the signaled
    /// state.
    ///
    /// If the `wait` parameter is [`TRUE`], [`WSAGetOverlappedResult`] determines whether the
    /// pending operation has been completed by waiting for the event object to be in the signaled
    /// state. A client may set the `wait` parameter to [`TRUE`], but only if it selected
    /// event-based completion notification when the I/O operation was requested. If another form
    /// of notification was selected, the usage of the `event` parameter of the [`WSAOVERLAPPED`]
    /// structure is different, and setting `wait` to [`TRUE`] causes unpredictable results.
    ///
    /// If the [`WSAGetOverlappedResult`] function is called with the `overlapped`, `cb_transfer`,
    /// or `flags` parameter set to a [`null_mut`] pointer on Windows Vista, this will result in an
    /// access violation. If the [`WSAGetOverlappedResult`] function is called with the
    /// `overlapped`, `cb_transfer`, or `flags` parameter set to a [`null_mut`] pointer on Windows
    /// Server 2003 and earlier, this will result in the [`WSAEFAULT`] error code being returned.
    pub fn WSAGetOverlappedResult(
        s: SOCKET,
        overlapped: LPWSAOVERLAPPED,
        cb_transfer: LPDWORD,
        wait: BOOL,
        flags: LPDWORD,
    ) -> BOOL;
}
