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
    RegConnectRegistry, RegConnectRegistryW, RegCopyTree, RegCopyTreeW, RegCreateKeyEx,
    RegCreateKeyExW, RegCreateKeyTransacted, RegCreateKeyTransactedW, RegDeleteKeyTransacted,
    RegDeleteKeyTransactedW, RegLoadKey, RegLoadKeyW, RegOpenCurrentUser, RegOpenKeyEx,
    RegOpenKeyExW, RegOpenKeyTransacted, RegOpenKeyTransactedW, RegReplaceKey, RegReplaceKeyW,
    RegRestoreKey, RegRestoreKeyW, RegSaveKey, RegSaveKeyEx, RegSaveKeyExW, RegSaveKeyW,
    RegSetKeyValue, RegSetKeyValueW, RegSetValueEx, RegSetValueExW,
};
pub use types::{LSTATUS, REGSAM};
