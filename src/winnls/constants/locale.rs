use std::ffi::c_int;

/// Maximum length of a locale name. The maximum number of characters allowed for this string is
/// 85, including a terminating null character. Note: Your application must use the constant for
/// the maximum locale name length, instead of hard-coding the value "85".
pub const LOCALE_NAME_MAX_LENGTH: c_int = 85;
