//! System information
//!
//! This header is used by multiple technologies such as:
//!  - System Services
//!  - Windows Sidebar

mod constants;
mod functions;
mod structures;
mod types;

pub use constants::*;
pub use functions::GetSystemInfo;
pub use structures::{SYSTEM_INFO, SYSTEM_INFO_PROCESSOR_ARCHITECTURE, SYSTEM_INFO_UNION};
pub use types::LPSYSTEM_INFO;
