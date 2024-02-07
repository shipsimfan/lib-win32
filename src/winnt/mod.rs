//! This header is used by multiple technologies such as:
//!  - Application Installation and Servicing
//!  - Backup
//!  - Data Access and Storage
//!  - Developer Notes
//!  - DXGI
//!  - Hardware Counter Profiling
//!  - Internationalization for Windows Applications
//!  - Kernel-Mode Driver Reference
//!  - Menus and Other Resources
//!  - Security and Identity
//!  - System Services
//!  - The Windows Shell
//!  - Windows Management Instrumentation
//!  - Windows Runtime C++ reference

mod types;

pub use types::{HRESULT, LANGID, LONG, LPTSTR, LPWSTR, TCHAR, WCHAR};
