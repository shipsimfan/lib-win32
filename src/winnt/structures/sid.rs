use crate::{BYTE, DWORD, SID_IDENTIFIER_AUTHORITY};

/// The security identifier ([`SID`]) structure is a variable-length structure used to uniquely
/// identify users or groups.
///
/// Applications should not modify a [`SID`] directly.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SID {
    #[allow(missing_docs)]
    pub revision: BYTE,

    #[allow(missing_docs)]
    pub sub_authority_count: BYTE,

    #[allow(missing_docs)]
    pub identifier_authority: SID_IDENTIFIER_AUTHORITY,

    #[allow(missing_docs)]
    pub sub_authority: [DWORD; 1],
}
