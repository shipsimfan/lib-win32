use crate::{d3d11::D3D11_AUTHENTICATED_CONFIGURE_INPUT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE;

/// Contains input data for a [`D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE`] command.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    /// A [`D3D11_AUTHENTICATED_CONFIGURE_INPUT`] structure that contains the command GUID and
    /// other data.
    pub parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,

    /// The initial sequence number for queries.
    pub start_sequence_query: UINT,

    /// The initial sequence number for commands.
    pub start_sequence_configure: UINT,
}

impl Default for D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_CONFIGURE_INITIALIZE_INPUT {
            parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT::default(),
            start_sequence_query: 0,
            start_sequence_configure: 0,
        }
    }
}
