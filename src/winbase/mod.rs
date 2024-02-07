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

pub use constants::*;
pub use functions::{FormatMessage, FormatMessageW, LocalAlloc};
