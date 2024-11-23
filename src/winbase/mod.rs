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
    FormatMessage, FormatMessageW, GetCommPorts, GetCommState, GlobalAlloc, GlobalFlags,
    GlobalFree, GlobalHandle, GlobalLock, GlobalReAlloc, GlobalSize, GlobalUnlock, LocalAlloc,
    LocalFlags, LocalFree, LocalHandle, LocalLock, LocalReAlloc, LocalSize, LocalUnlock,
    SetCommState,
};
pub use structures::{
    DCB, FILETIME, OVERLAPPED, OVERLAPPED_UNION, OVERLAPPED_UNION_STRUCT, SECURITY_ATTRIBUTES,
    SYSTEMTIME,
};
pub use types::{LPDCB, LPFILETIME, LPOVERLAPPED, LPSECURITY_ATTRIBUTES, LPSYSTEMTIME, PFILETIME};
