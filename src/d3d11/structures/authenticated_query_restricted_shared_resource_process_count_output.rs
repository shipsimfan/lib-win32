use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_OUTPUT, UINT};

#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT;

/// Contains the response to a
/// [`D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_OUTPUT`] structure that contains a Message Authentication
    /// Code (MAC) and other data.
    pub output: D3D11_AUTHENTICATED_QUERY_OUTPUT,

    /// The number of processes that are allowed to open shared resources that have restricted
    /// access. A process cannot open such a resource unless the process has been granted access.
    pub restricted_shared_resource_process_count: UINT,
}

impl Default for D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_RESTRICTED_SHARED_RESOURCE_PROCESS_COUNT_OUTPUT {
            output: D3D11_AUTHENTICATED_QUERY_OUTPUT::default(),
            restricted_shared_resource_process_count: 0,
        }
    }
}
