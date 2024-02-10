use crate::{BYTE, PACL, PSID, SECURITY_DESCRIPTOR_CONTROL};

/// The [`SECURITY_DESCRIPTOR`] structure contains the security information associated with an
/// object. Applications use this structure to set and query an object's security status.
///
/// Because the internal format of a security descriptor can vary, we recommend that applications
/// not modify the [`SECURITY_DESCRIPTOR`] structure directly.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SECURITY_DESCRIPTOR {
    #[allow(missing_docs)]
    pub revision: BYTE,

    #[allow(missing_docs)]
    pub sbz1: BYTE,

    #[allow(missing_docs)]
    pub control: SECURITY_DESCRIPTOR_CONTROL,

    #[allow(missing_docs)]
    pub owner: PSID,

    #[allow(missing_docs)]
    pub group: PSID,

    #[allow(missing_docs)]
    pub sacl: PACL,

    #[allow(missing_docs)]
    pub dacl: PACL,
}
