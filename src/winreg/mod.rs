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
    RegCloseKey, RegConnectRegistry, RegConnectRegistryW, RegCopyTree, RegCopyTreeW,
    RegCreateKeyEx, RegCreateKeyExW, RegCreateKeyTransacted, RegCreateKeyTransactedW,
    RegDeleteKeyEx, RegDeleteKeyExW, RegDeleteKeyTransacted, RegDeleteKeyTransactedW,
    RegLoadAppKey, RegLoadAppKeyW, RegLoadKey, RegLoadKeyW, RegOpenCurrentUser, RegOpenKeyEx,
    RegOpenKeyExW, RegOpenKeyTransacted, RegOpenKeyTransactedW, RegReplaceKey, RegReplaceKeyW,
    RegRestoreKey, RegRestoreKeyW, RegSaveKey, RegSaveKeyEx, RegSaveKeyExW, RegSaveKeyW,
    RegSetKeySecurity, RegSetKeyValue, RegSetKeyValueW, RegSetValueEx, RegSetValueExW,
};
pub use types::{LSTATUS, REGSAM};
