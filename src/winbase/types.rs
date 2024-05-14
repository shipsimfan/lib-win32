use crate::{FILETIME, SECURITY_ATTRIBUTES};

/// A pointer to a [`SECURITY_ATTRIBUTES`] structure
#[allow(non_camel_case_types)]
pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;

/// A pointer to a [`FILETIME`] structure
#[allow(non_camel_case_types)]
pub type PFILETIME = *mut FILETIME;
