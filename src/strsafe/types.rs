/// A pointer to a constant null-terminated string of 16-bit Unicode characters
#[allow(non_camel_case_types)]
pub type STRSAFE_LPCWSTR = *const u16;

/// A pointer to a null-terminated string of 16-bit Unicode characters
#[allow(non_camel_case_types)]
pub type STRSAFE_LPWSTR = *mut u16;
