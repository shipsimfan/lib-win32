use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_OUTPUT, GUID, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID;

/// Contains the response to a [`D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_OUTPUT`] structure that contains a Message Authentication
    /// Code (MAC) and other data.
    pub output: D3D11_AUTHENTICATED_QUERY_OUTPUT,

    /// The index of the encryption [`GUID`].
    pub encryption_guid_index: UINT,

    /// A [`GUID`] that specifies a supported encryption type.
    pub encryption_guid: GUID,
}

impl Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_OUTPUT {
            output: D3D11_AUTHENTICATED_QUERY_OUTPUT::default(),
            encryption_guid_index: 0,
            encryption_guid: GUID::default(),
        }
    }
}
