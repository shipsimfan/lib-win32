use crate::{
    d3d11::{D3D11_AUTHENTICATED_QUERY_OUTPUT, D3D11_BUS_TYPE},
    BOOL,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES, TRUE};

/// Contains the response to a [`D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_ATTRIBUTES`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_OUTPUT`] structure that contains a Message Authentication
    /// Code (MAC) and other data.
    pub output: D3D11_AUTHENTICATED_QUERY_OUTPUT,

    /// A bitwise OR of flags from the [`D3D11_BUS_TYPE`] enumeration.
    pub bus_type: D3D11_BUS_TYPE,

    /// If [`TRUE`], contiguous blocks of video memory may be accessible to the CPU or the bus.
    pub accessible_in_contiguous_blocks: BOOL,

    /// If [`TRUE`], non-contiguous blocks of video memory may be accessible to the CPU or the bus.
    pub accessible_in_non_contiguous_blocks: BOOL,
}

impl Default for D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_ACCESSIBILITY_OUTPUT {
            output: D3D11_AUTHENTICATED_QUERY_OUTPUT::default(),
            bus_type: D3D11_BUS_TYPE::Other,
            accessible_in_contiguous_blocks: 0,
            accessible_in_non_contiguous_blocks: 0,
        }
    }
}
