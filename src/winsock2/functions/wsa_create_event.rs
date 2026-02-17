use crate::winsock2::WSAEVENT;

// rustdoc imports
#[allow(unused_imports)]
use crate::winsock2::{WSAGetLastError, WSA_INVALID_EVENT};

#[link(name = "Ws2_32")]
unsafe extern "system" {
    /// The [`WSACreateEvent`] function creates a new event object.
    ///
    /// # Return Value
    /// If no error occurs, [`WSACreateEvent`] returns the handle of the event object. Otherwise,
    /// the return value is [`WSA_INVALID_EVENT`]. To get extended error information, call
    /// [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`WSACreateEvent`] function creates a manual-reset event object with an initial state
    /// of nonsignaled. The handle of the event object returned cannot be inherited by child
    /// processes. The event object is unnamed.
    ///
    /// The [`WSASetEvent`] function can be called to set the state of the event object to
    /// signaled. The [`WSAResetEvent`] function can be called to set the state of the event object
    /// to nonsignaled. When an event object is no longer needed, the [`WSACloseEvent`] function
    /// should be called to free the resources associated with the event object.
    ///
    /// Windows Sockets 2 event objects are system objects in Windows environments. Therefore, if a
    /// Windows application wants to use an auto-reset event rather than a manual-reset event, the
    /// application can call the [`CreateEvent`] function directly. The scope of an event object is
    /// limited to the process in which it is created.
    pub fn WSACreateEvent() -> WSAEVENT;
}
