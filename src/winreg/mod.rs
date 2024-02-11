//! Windows registry
//!
//! This header is used by multiple technologies:
//!  - Developer Notes
//!  - Hyper-V WMI Provider
//!  - Security and Identity
//!  - System Services

mod constants;
mod functions;
mod types;

pub use constants::*;
pub use functions::{RegCopyTree, RegCopyTreeW, RegOpenCurrentUser, RegSaveKeyEx, RegSaveKeyExW};
pub use types::{LSTATUS, REGSAM};
