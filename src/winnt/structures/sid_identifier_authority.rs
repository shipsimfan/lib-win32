use crate::BYTE;

// rustdoc imports
#[allow(unused_imports)]
use crate::SID;

/// The [`SID_IDENTIFIER_AUTHORITY`] structure represents the top-level authority of a security
/// identifier ([`SID`]).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SID_IDENTIFIER_AUTHORITY {
    /// An array of 6 bytes specifying a [`SID`]'s top-level authority.
    pub value: [BYTE; 6],
}
