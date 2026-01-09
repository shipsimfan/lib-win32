use crate::DWORD;

/// The [`TOKEN_ELEVATION`] structure indicates whether a token has elevated privileges.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct TOKEN_ELEVATION {
    /// A nonzero value if the token has elevated privileges; otherwise, a zero value.
    pub token_is_elevated: DWORD,
}

impl Default for TOKEN_ELEVATION {
    fn default() -> Self {
        TOKEN_ELEVATION {
            token_is_elevated: 0,
        }
    }
}
