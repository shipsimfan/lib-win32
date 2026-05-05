use std::ffi::c_ushort;

/// integer type capable of storing any [`wchar_t`]
#[allow(non_camel_case_types)]
pub type wint_t = c_ushort;
