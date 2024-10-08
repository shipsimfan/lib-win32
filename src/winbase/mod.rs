//! This header is used by multiple technologies such as:
//!  - Application Installation and Servicing
//!  - Application Recovery and Restart
//!  - Backup
//!  - Data Access and Storage
//!  - Data Exchange
//!  - Developer Notes
//!  - eventlogprov
//!  - Hardware Counter Profiling
//!  - Internationalization for Windows Applications
//!  - Menus and Other Resources
//!  - Operation Recorder
//!  - Remote Desktop Services
//!  - Security and Identity
//!  - System Services
//!  - Window Stations and Desktops
//!  - Windows and Messages

mod constants;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use functions::{
    FormatMessage, FormatMessageW, GlobalAlloc, GlobalFlags, GlobalFree, GlobalHandle, GlobalLock,
    GlobalReAlloc, GlobalSize, GlobalUnlock, LocalAlloc, LocalFlags, LocalFree, LocalHandle,
    LocalLock, LocalReAlloc, LocalSize, LocalUnlock,
};
pub use structures::{FILETIME, SECURITY_ATTRIBUTES, SYSTEMTIME};
pub use types::{LPFILETIME, LPSECURITY_ATTRIBUTES, LPSYSTEMTIME, PFILETIME};
