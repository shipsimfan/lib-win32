use crate::winsock2::WSAEVENT;
use std::ptr::null_mut;

/// An invalid WSA event
pub const WSA_INVALID_EVENT: WSAEVENT = null_mut();
