use crate::SYSTEM_INFO;

/// A pointer to a [`SYSTEM_INFO`] structure
#[allow(non_camel_case_types)]
pub type LPSYSTEM_INFO = *mut SYSTEM_INFO;
