use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_INPUT, HANDLE, UINT};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_OUTPUT_ID, GUID};

/// Contains input data for a [`D3D11_AUTHENTICATED_QUERY_OUTPUT_ID`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_INPUT`] structure that contains the [`GUID`] for the query
    /// and other data.
    pub input: D3D11_AUTHENTICATED_QUERY_INPUT,

    /// A handle to the device.
    pub device_handle: HANDLE,

    /// A handle to the cryptographic session.
    pub crypto_session_handle: HANDLE,

    /// The index of the output ID.
    pub output_id_index: UINT,
}

impl Default for D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_OUTPUT_ID_INPUT {
            input: D3D11_AUTHENTICATED_QUERY_INPUT::default(),
            device_handle: null_mut(),
            crypto_session_handle: null_mut(),
            output_id_index: 0,
        }
    }
}
