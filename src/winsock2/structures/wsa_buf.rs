use crate::{CHAR, ULONG};
use std::ptr::null_mut;

/// The [`WSABUF`] structure enables the creation or manipulation of a data buffer used by some
/// Winsock functions.
#[repr(C)]
#[derive(Debug, Clone)]
pub struct WSABUF {
    /// The length of the buffer, in bytes.
    pub len: ULONG,

    /// A pointer to the buffer.
    pub buf: *mut CHAR,
}

impl Default for WSABUF {
    fn default() -> Self {
        WSABUF {
            len: 0,
            buf: null_mut(),
        }
    }
}
