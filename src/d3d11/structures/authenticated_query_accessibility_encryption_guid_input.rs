use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_INPUT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID, GUID};

/// Contains input data for a [`D3D11_AUTHENTICATED_QUERY_ENCRYPTION_WHEN_ACCESSIBLE_GUID`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_INPUT`] structure that contains the [`GUID`] for the query
    /// and other data.
    pub input: D3D11_AUTHENTICATED_QUERY_INPUT,

    /// The index of the encryption [`GUID`].
    pub encryption_guid_index: UINT,
}

impl Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ENCRYPTION_GUID_INPUT {
            input: D3D11_AUTHENTICATED_QUERY_INPUT::default(),
            encryption_guid_index: 0,
        }
    }
}
