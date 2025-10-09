use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_INPUT, UINT};

#[allow(unused_imports)]
use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS, GUID};

/// Contains input data for a [`D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_INPUT`] structure that contains the [`GUID`] for the query
    /// and other data.
    pub input: D3D11_AUTHENTICATED_QUERY_INPUT,

    /// The index of the process.
    pub process_index: UINT,
}

impl Default for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_INPUT {
            input: D3D11_AUTHENTICATED_QUERY_INPUT::default(),
            process_index: 0,
        }
    }
}
