use crate::{winsock2::WSAEVENT, DWORD};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::OVERLAPPED;

/// The [`WSAOVERLAPPED`] structure provides a communication medium between the initiation of an
/// overlapped I/O operation and its subsequent completion. The [`WSAOVERLAPPED`] structure is
/// compatible with the Windows [`OVERLAPPED`] structure.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WSAOVERLAPPED {
    /// Reserved for internal use. The `internal` member is used internally by the entity that
    /// implements overlapped I/O. For service providers that create sockets as installable file
    /// system (IFS) handles, this parameter is used by the underlying operating system. Other
    /// service providers (non-IFS providers) are free to use this parameter as necessary.
    pub internal: DWORD,

    /// Reserved. Used internally by the entity that implements overlapped I/O. For service
    /// providers that create sockets as IFS handles, this parameter is used by the underlying
    /// operating system. NonIFS providers are free to use this parameter as necessary.
    pub internal_high: DWORD,

    /// Reserved for use by service providers.
    pub offset: DWORD,

    /// Reserved for use by service providers.
    pub offset_high: DWORD,

    /// If an overlapped I/O operation is issued without an I/O completion routine (the operation's
    /// `completion_routine` parameter is set to [`null_mut`]), then this parameter should either
    /// contain a valid handle to a [`WSAEVENT`] object or be [`null_mut`]. If the
    /// `completion_routine` parameter of the call is non-null then applications are free to use
    /// this parameter as necessary.
    pub event: WSAEVENT,
}

impl Default for WSAOVERLAPPED {
    fn default() -> Self {
        WSAOVERLAPPED {
            internal: 0,
            internal_high: 0,
            offset: 0,
            offset_high: 0,
            event: null_mut(),
        }
    }
}
