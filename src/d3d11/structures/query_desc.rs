use crate::{d3d11::D3D11_QUERY, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_QUERY_MISC_FLAG;

/// Describes a query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_QUERY_DESC {
    /// Type of query (see [`D3D11_QUERY`]).
    pub query: D3D11_QUERY,

    /// Miscellaneous flags (see [`D3D11_QUERY_MISC_FLAG`]).
    pub misc_flags: UINT,
}

impl Default for D3D11_QUERY_DESC {
    fn default() -> Self {
        D3D11_QUERY_DESC {
            query: D3D11_QUERY::Event,
            misc_flags: 0,
        }
    }
}
