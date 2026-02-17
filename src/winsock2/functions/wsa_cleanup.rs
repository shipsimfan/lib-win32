use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::{closesocket, WSAGetLastError, WSAStartup, SOCKET_ERROR};

#[link(name = "Ws2_32")]
unsafe extern "system" {
    /// The WSACleanup function terminates use of the Winsock 2 DLL.
    ///
    /// # Return Value
    /// The return value is zero if the operation was successful. Otherwise, the value
    /// [`SOCKET_ERROR`] is returned, and a specific error number can be retrieved by calling
    /// [`WSAGetLastError`].
    ///
    /// In a multithreaded environment, [`WSACleanup`] terminates Windows Sockets operations for
    /// all threads.
    ///
    /// # Remarks
    /// An application or DLL is required to perform a successful [`WSAStartup`] call before it can
    /// use Windows Sockets services. When it has completed the use of Windows Sockets, the
    /// application or DLL must call [`WSACleanup`] to deregister itself from a Windows Sockets
    /// implementation and allow the implementation to free any resources allocated on behalf of
    /// the application or DLL.
    ///
    /// When [`WSACleanup`] is called, any pending blocking or asynchronous Windows Sockets calls
    /// issued by any thread in this process are canceled without posting any notification messages
    /// or without signaling any event objects. Any pending overlapped send or receive operations
    /// ([`WSASend`], [`WSASendTo`], [`WSARecv`], or [`WSARecvFrom`] with an overlapped socket, for
    /// example) issued by any thread in this process are also canceled without setting the event
    /// object or invoking the completion routine, if one was specified. In this case, the pending
    /// overlapped operations fail with the error status [`WSA_OPERATION_ABORTED`].
    ///
    /// Sockets that were open when [`WSACleanup`] was called are reset and automatically
    /// deallocated as if [`closesocket`] were called. Sockets that have been closed with
    /// [`closesocket`] but that still have pending data to be sent can be affected when
    /// [`WSACleanup`] is called. In this case, the pending data can be lost if the `WS2_32.DLL` is
    /// unloaded from memory as the application exits. To ensure that all pending data is sent, an
    /// application should use shutdown to close the connection, then wait until the close
    /// completes before calling [`closesocket`] and [`WSACleanup`]. All resources and internal
    /// state, such as queued unposted or posted messages, must be deallocated so as to be
    /// available to the next user.
    ///
    /// There must be a call to [`WSACleanup`] for each successful call to [`WSAStartup`]. Only the
    /// final [`WSACleanup`] function call performs the actual cleanup. The preceding calls simply
    /// decrement an internal reference count in the WS2_32.DLL.
    ///
    /// In Windows Sockets 1.1, attempting to call [`WSACleanup`] from within a blocking hook and
    /// then failing to check the return code was a common programming error. If a Winsock 1.1
    /// application needs to quit while a blocking call is outstanding, the application has to
    /// first cancel the blocking call with [`WSACancelBlockingCall`] then issue the [`WSACleanup`]
    /// call once control has been returned to the application. In Windows Sockets 2, this issue
    /// does not exist and the [`WSACancelBlockingCall`] function has been removed.
    ///
    /// The [`WSACleanup`] function typically leads to protocol-specific helper DLLs being
    /// unloaded. As a result, the [`WSACleanup`] function should not be called from the `DllMain`
    /// function in a application DLL. This can potentially cause deadlocks.
    pub fn WSACleanup() -> c_int;
}
