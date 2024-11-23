use crate::{DCB, FILETIME, OVERLAPPED, SECURITY_ATTRIBUTES, SYSTEMTIME};

/// A pointer to a [`DCB`] structure
pub type LPDCB = *mut DCB;

/// A pointer to a [`FILETIME`] structure
pub type LPFILETIME = *mut FILETIME;

/// A pointer to an [`OVERLAPPED`] structure
pub type LPOVERLAPPED = *mut OVERLAPPED;

/// A pointer to a [`SECURITY_ATTRIBUTES`] structure
#[allow(non_camel_case_types)]
pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;

/// A pointer to a [`SYSTEMTIME`] structure
pub type LPSYSTEMTIME = *mut SYSTEMTIME;

/// A pointer to a [`FILETIME`] structure
#[allow(non_camel_case_types)]
pub type PFILETIME = *mut FILETIME;
