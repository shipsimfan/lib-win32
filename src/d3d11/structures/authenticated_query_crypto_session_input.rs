use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_INPUT, HANDLE};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION, GUID};

/// Contains input data for a [`D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_INPUT`] structure that contains the [`GUID`] for the query
    /// and other data.
    pub input: D3D11_AUTHENTICATED_QUERY_INPUT,

    /// A handle to a decoder device.
    pub decoder_handle: HANDLE,
}

impl Default for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_INPUT {
            input: D3D11_AUTHENTICATED_QUERY_INPUT::default(),
            decoder_handle: null_mut(),
        }
    }
}
