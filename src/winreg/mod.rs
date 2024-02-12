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
pub use functions::{
    RegCopyTree, RegCopyTreeW, RegCreateKeyEx, RegCreateKeyExW, RegLoadKey, RegLoadKeyW,
    RegOpenCurrentUser, RegReplaceKey, RegReplaceKeyW, RegRestoreKey, RegRestoreKeyW, RegSaveKey,
    RegSaveKeyEx, RegSaveKeyExW, RegSaveKeyW,
};
pub use types::{LSTATUS, REGSAM};
