use crate::{BYTE, WORD};

// rustdoc imports
#[allow(unused_imports)]
use crate::{ACL_REVISION, ACL_REVISION_DS};

/// The [`ACL`] structure is the header of an access control list (ACL). A complete ACL consists of
/// an [`ACL`] structure followed by an ordered list of zero or more access control entries (ACEs).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ACL {
    /// Specifies the revision level of the [`ACL`]. This value should be [`ACL_REVISION`], unless
    /// the [`ACL`] contains an object-specific ACE, in which case this value must be
    /// [`ACL_REVISION_DS`]. All ACEs in an [`ACL`] must be at the same revision level.
    pub acl_revision: BYTE,

    /// Specifies a zero byte of padding that aligns the `acl_revision` member on a 16-bit
    /// boundary.
    pub sbz1: BYTE,

    /// Specifies the size, in bytes, of the [`ACL`]. This value includes the [`ACL`] structure,
    /// all the ACEs, and the potential unused memory.
    pub acl_size: WORD,

    /// Specifies the number of ACEs stored in the [`ACL`].
    pub ace_count: WORD,

    /// Specifies two zero-bytes of padding that align the [`ACL`] structure on a 32-bit boundary.
    pub sbz2: WORD,
}
