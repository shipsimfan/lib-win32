use crate::{
    REGSAM, STANDARD_RIGHTS_ALL, STANDARD_RIGHTS_READ, STANDARD_RIGHTS_WRITE, SYNCHRONIZE,
};

/// Required to query the values of a registry key.
pub const KEY_QUERY_VALUE: REGSAM = 0x0001;

/// Required to create, delete, or set a registry value.
pub const KEY_SET_VALUE: REGSAM = 0x0002;

/// Required to create a subkey of a registry key.
pub const KEY_CREATE_SUB_KEY: REGSAM = 0x0004;

/// Required to enumerate the subkeys of a registry key.
pub const KEY_ENUMERATE_SUB_KEYS: REGSAM = 0x0008;

/// Required to request change notifications for a registry key or for subkeys of a registry key.
pub const KEY_NOTIFY: REGSAM = 0x0010;

/// Reserved for system use.
pub const KEY_CREATE_LINK: REGSAM = 0x0020;

/// Indicates that an application on 64-bit Windows should operate on the 32-bit registry view.
/// This flag is ignored by 32-bit Windows. For more information, see Accessing an Alternate
/// Registry View.
///
/// This flag must be combined using the OR operator with the other flags in this table that either
/// query or access registry values.
///
/// Windows 2000: This flag is not supported.
pub const KEY_WOW64_32KEY: REGSAM = 0x0200;

/// Indicates that an application on 64-bit Windows should operate on the 64-bit registry view.
/// This flag is ignored by 32-bit Windows. For more information, see Accessing an Alternate
/// Registry View.
///
/// This flag must be combined using the OR operator with the other flags in this table that either
/// query or access registry values.
///
/// Windows 2000: This flag is not supported.
pub const KEY_WOW64_64KEY: REGSAM = 0x0100;

/// Combines the [`STANDARD_RIGHTS_READ`], [`KEY_QUERY_VALUE`], [`KEY_ENUMERATE_SUB_KEYS`], and
/// [`KEY_NOTIFY`] values.
pub const KEY_READ: REGSAM =
    (STANDARD_RIGHTS_READ | KEY_QUERY_VALUE | KEY_ENUMERATE_SUB_KEYS | KEY_NOTIFY) & !SYNCHRONIZE;

/// Combines the [`STANDARD_RIGHTS_WRITE`], [`KEY_SET_VALUE`], and [`KEY_CREATE_SUB_KEY`] access
/// rights.
pub const KEY_WRITE: REGSAM =
    (STANDARD_RIGHTS_WRITE | KEY_SET_VALUE | KEY_CREATE_SUB_KEY) & !SYNCHRONIZE;

/// Equivalent to [`KEY_READ`]
pub const KEY_EXECUTE: REGSAM = KEY_READ & !SYNCHRONIZE;

/// Combines the [`STANDARD_RIGHTS_REQUIRED`], [`KEY_QUERY_VALUE`], [`KEY_SET_VALUE`],
/// [`KEY_CREATE_SUB_KEY`], [`KEY_ENUMERATE_SUB_KEYS`], [`KEY_NOTIFY`], and [`KEY_CREATE_LINK`]
/// access rights.
pub const KEY_ALL_ACCESS: REGSAM = (STANDARD_RIGHTS_ALL
    | KEY_QUERY_VALUE
    | KEY_SET_VALUE
    | KEY_CREATE_SUB_KEY
    | KEY_ENUMERATE_SUB_KEYS
    | KEY_NOTIFY
    | KEY_CREATE_LINK)
    & !SYNCHRONIZE;
