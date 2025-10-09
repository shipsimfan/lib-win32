use crate::d3d11::{D3D11_AUTHENTICATED_PROTECTION_FLAGS, D3D11_AUTHENTICATED_QUERY_OUTPUT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_QUERY_PROTECTION;

/// Contains the response to a [`D3D11_AUTHENTICATED_QUERY_PROTECTION`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_OUTPUT`] structure that contains a Message Authentication
    /// Code (MAC) and other data.
    pub output: D3D11_AUTHENTICATED_QUERY_OUTPUT,

    /// A [`D3D11_AUTHENTICATED_PROTECTION_FLAGS`] union that specifies the protection level.
    pub protection_flags: D3D11_AUTHENTICATED_PROTECTION_FLAGS,
}

impl Default for D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_PROTECTION_OUTPUT {
            output: D3D11_AUTHENTICATED_QUERY_OUTPUT::default(),
            protection_flags: D3D11_AUTHENTICATED_PROTECTION_FLAGS::default(),
        }
    }
}
