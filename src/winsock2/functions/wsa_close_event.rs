use crate::{winsock2::WSAEVENT, BOOL};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    winsock2::{WSACreateEvent, WSAGetLastError},
    FALSE, TRUE,
};

#[link(name = "Ws2_32")]
unsafe extern "system" {
    /// The [`WSACloseEvent`] function closes an open event object handle.
    ///
    /// # Parameters
    ///  * `event` - Object handle identifying the open event.
    ///
    /// # Return Value
    /// If the function succeeds, the return value is [`TRUE`].
    ///
    /// If the function fails, the return value is [`FALSE`]. To get extended error information,
    /// call [`WSAGetLastError`].
    ///
    /// # Remarks
    /// The [`WSACloseEvent`] function closes the handle to an event object and frees resources
    /// associated with the event object. This function is used to close a handle created by the
    /// [`WSACreateEvent`] function. Once the handle to the event object is closed, further
    /// references to this handle will fail with the error [`WSA_INVALID_HANDLE`].
    pub fn WSACloseEvent(event: WSAEVENT) -> BOOL;
}
