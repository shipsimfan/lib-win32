use crate::d3d11::{D3D11_AUTHENTICATED_CHANNEL_TYPE, D3D11_AUTHENTICATED_QUERY_OUTPUT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE;

/// Contains the response to a [`D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_OUTPUT`] structure that contains a Message Authentication
    /// Code (MAC) and other data.
    pub output: D3D11_AUTHENTICATED_QUERY_OUTPUT,

    /// A [`D3D11_AUTHENTICATED_CHANNEL_TYPE`] value that specifies the channel type.
    pub channel_type: D3D11_AUTHENTICATED_CHANNEL_TYPE,
}

impl Default for D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_CHANNEL_TYPE_OUTPUT {
            output: D3D11_AUTHENTICATED_QUERY_OUTPUT::default(),
            channel_type: D3D11_AUTHENTICATED_CHANNEL_TYPE::D3D11,
        }
    }
}
