use crate::DWORD;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    RegGetValue, ERROR_INVALID_PARAMETER, ERROR_SUCCESS, REG_BINARY, REG_DWORD, REG_EXPAND_SZ,
    REG_MULTI_SZ, REG_NONE, REG_QWORD, REG_SZ,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Restrict type to [`REG_NONE`] (other data types will not return [`ERROR_SUCCESS`])
pub const RRF_RT_REG_NONE: DWORD = 0x00000001;

/// Restrict type to [`REG_SZ`] (other data types will not return [`ERROR_SUCCESS`]) (automatically
/// converts [`REG_EXPAND_SZ`] to [`REG_SZ`] unless [`RRF_NOEXPAND`] is specified)
pub const RRF_RT_REG_SZ: DWORD = 0x00000002;

/// Restrict type to [`REG_EXPAND_SZ`] (other data types will not return [`ERROR_SUCCESS`]) (must
/// specify [`RRF_NOEXPAND`] or [`RegGetValue`] will fail with [`ERROR_INVALID_PARAMETER`])
pub const RRF_RT_REG_EXPAND_SZ: DWORD = 0x00000004;

/// Restrict type to [`REG_BINARY`] (other data types will not return [`ERROR_SUCCESS`])
pub const RRF_RT_REG_BINARY: DWORD = 0x00000008;

/// Restrict type to [`REG_DWORD`] (other data types will not return [`ERROR_SUCCESS`])
pub const RRF_RT_REG_DWORD: DWORD = 0x00000010;

/// Restrict type to [`REG_MULTI_SZ`] (other data types will not return [`ERROR_SUCCESS`])
pub const RRF_RT_REG_MULTI_SZ: DWORD = 0x00000020;

/// Restrict type to [`REG_QWORD`] (other data types will not return [`ERROR_SUCCESS`])
pub const RRF_RT_REG_QWORD: DWORD = 0x00000040;

/// Restrict type to *32-bit* [`RRF_RT_REG_BINARY`] or [`RRF_RT_REG_DWORD`] (other data types will
/// not return [`ERROR_SUCCESS`])
pub const RRF_RT_DWORD: DWORD = RRF_RT_REG_BINARY | RRF_RT_REG_DWORD;

/// Restrict type to *64-bit* [`RRF_RT_REG_BINARY`] or [`RRF_RT_REG_DWORD`] (other data types will
/// not return [`ERROR_SUCCESS`])
pub const RRF_RT_QWORD: DWORD = RRF_RT_REG_BINARY | RRF_RT_REG_QWORD;

/// No type restriction
pub const RRF_RT_ANY: DWORD = 0x0000ffff;

/// When opening the subkey (if provided) force open from the 64-bit location (only one
/// `SUBKEY_WOW64*` flag can be set or [`RegGetValue`] will fail with [`ERROR_INVALID_PARAMETER`])
pub const RRF_SUBKEY_WOW6464KEY: DWORD = 0x00010000;

/// When opening the subkey (if provided) force open from the 32-bit location (only one
/// `SUBKEY_WOW64*` flag can be set or [`RegGetValue`] will fail with [`ERROR_INVALID_PARAMETER`])
pub const RRF_SUBKEY_WOW6432KEY: DWORD = 0x00020000;

/// Do not automatically expand environment strings if value is of type [`REG_EXPAND_SZ`]
pub const RRF_NOEXPAND: DWORD = 0x10000000;

/// If `data` is not [`null_mut`], set content to all zeros on failure
pub const RRF_ZEROONFAILURE: DWORD = 0x20000000;
