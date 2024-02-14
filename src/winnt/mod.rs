//! Windows NT Definitions
//!
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

mod constants;
mod enumerations;
mod structures;
mod types;

pub use constants::*;
pub use enumerations::MEM_EXTENDED_PARAMETER_TYPE;
pub use structures::{
    ACL, LARGE_INTEGER, LARGE_INTEGER_DUMMY, MEM_ADDRESS_REQUIREMENTS, MEM_EXTENDED_PARAMETER,
    MEM_EXTENDED_PARAMETER_UNION, SECURITY_DESCRIPTOR, SID, SID_IDENTIFIER_AUTHORITY,
};
pub use types::{
    ACCESS_MASK, HANDLE, HRESULT, LANGID, LONG, LONGLONG, LPCWSTR, LPTSTR, LPWSTR, PACL,
    PSECURITY_DESCRIPTOR, PSID, PVOID, SECURITY_DESCRIPTOR_CONTROL, SECURITY_INFORMATION, TCHAR,
    WCHAR,
};
