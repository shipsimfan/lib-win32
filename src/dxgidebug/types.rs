use crate::GUID;
use std::ffi::c_int;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgidebug::IDXGIInfoQueue;

/// Globally unique identifier ([`GUID`]) values that identify producers of debug messages.
///
/// # Remarks
/// Use these values with the [`IDXGIInfoQueue`] interface.
#[allow(non_camel_case_types)]
pub type DXGI_DEBUG_ID = GUID;

#[allow(missing_docs)]
#[allow(non_camel_case_types)]
pub type DXGI_INFO_QUEUE_MESSAGE_ID = c_int;
