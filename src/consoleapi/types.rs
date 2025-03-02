use crate::{BOOL, DWORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::SetConsoleCtrlHandler;

/// An application-defined function used with the [`SetConsoleCtrlHandler`] function.
#[allow(non_camel_case_types)]
pub type PHANDLER_ROUTINE = extern "system" fn(DWORD) -> BOOL;
