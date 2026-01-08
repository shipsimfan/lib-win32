use crate::wdm::{OSVERSIONINFOEXW, OSVERSIONINFOW};

/// [`OSVERSIONINFOW`]
#[allow(non_camel_case_types)]
pub type RTL_OSVERSIONINFOW = OSVERSIONINFOW;

/// [`OSVERSIONINFOEXW`]
#[allow(non_camel_case_types)]
pub type RTL_OSVERSIONINFOEXW = OSVERSIONINFOEXW;

/// A pointer to an [`RTL_OSVERSIONINFOW`]
#[allow(non_camel_case_types)]
pub type PRTL_OSVERSIONINFOW = *mut RTL_OSVERSIONINFOW;
