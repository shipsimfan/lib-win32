//! Windows Definitions
//!
//! This header is used by multiple technologies such as:
//!  - Display Devices Reference
//!  - High DPI
//!  - Windows Accessibility Features
//!  - Windows and Messages
//!  - Windows GDI

mod constants;
mod types;

pub use constants::*;
pub use types::{BOOL, HGLOBAL, HLOCAL, INT, LPCVOID, LPVOID, PDWORD, UINT, WORD};
