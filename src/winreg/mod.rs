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
    RegCreateKeyEx, RegCreateKeyExW, RegCreateKeyTransacted, RegCreateKeyTransactedW, RegDeleteKey,
    RegDeleteKeyEx, RegDeleteKeyExW, RegDeleteKeyTransacted, RegDeleteKeyTransactedW,
    RegDeleteKeyW, RegDeleteTree, RegDeleteTreeW, RegEnumKeyEx, RegEnumKeyExW, RegFlushKey,
    RegLoadAppKey, RegLoadAppKeyW, RegLoadKey, RegLoadKeyW, RegOpenCurrentUser, RegOpenKeyEx,
    RegOpenKeyExW, RegOpenKeyTransacted, RegOpenKeyTransactedW, RegReplaceKey, RegReplaceKeyW,
    RegRestoreKey, RegRestoreKeyW, RegSaveKey, RegSaveKeyEx, RegSaveKeyExW, RegSaveKeyW,
    RegSetKeySecurity, RegSetKeyValue, RegSetKeyValueW, RegSetValueEx, RegSetValueExW,
    RegUnLoadKey, RegUnLoadKeyW,
};
pub use types::{LSTATUS, REGSAM};
