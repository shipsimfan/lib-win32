use crate::{d3d11::D3D11_AUTHENTICATED_CONFIGURE_INPUT, GUID};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE;

/// Contains input data for a [`D3D11_AUTHENTICATED_CONFIGURE_ENCRYPTION_WHEN_ACCESSIBLE`] command.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_ACCESSIBLE_ENCRYPTION_INPUT {
    /// A [`D3D11_AUTHENTICATED_CONFIGURE_INPUT`] structure that contains the command GUID and
    /// other data.
    pub parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,

    /// A GUID that specifies the type of encryption to apply.
    pub encryption_guid: GUID,
}
