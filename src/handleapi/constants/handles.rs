use crate::{HANDLE, LONG_PTR};

/// An invalid handle
pub const INVALID_HANDLE_VALUE: HANDLE = (-1 as LONG_PTR) as HANDLE;
