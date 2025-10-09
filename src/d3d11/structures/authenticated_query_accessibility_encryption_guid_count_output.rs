use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_OUTPUT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT, GUID};

/// Contains the response to a [`D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID_COUNT`]
/// query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_OUTPUT`] structure that contains a Message Authentication
    /// Code (MAC) and other data.
    pub output: D3D11_AUTHENTICATED_QUERY_OUTPUT,

    /// The number of encryption [`GUID`]s.
    pub encryption_guid_count: UINT,
}

impl Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_COUNT_OUTPUT {
            output: D3D11_AUTHENTICATED_QUERY_OUTPUT::default(),
            encryption_guid_count: 0,
        }
    }
}
