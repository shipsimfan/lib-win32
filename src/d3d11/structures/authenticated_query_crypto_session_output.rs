use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_OUTPUT, HANDLE};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION;

/// Contains the response to a [`D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_OUTPUT`] structure that contains a Message Authentication
    /// Code (MAC) and other data.
    pub output: D3D11_AUTHENTICATED_QUERY_OUTPUT,

    /// A handle to a decoder device.
    pub decoder_handle: HANDLE,

    /// A handle to the cryptographic session that is associated with the decoder device.
    pub crypto_session_handle: HANDLE,

    /// A handle to the Direct3D device that is associated with the decoder device.
    pub device_handle: HANDLE,
}

impl Default for D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_CRYPTO_SESSION_OUTPUT {
            output: D3D11_AUTHENTICATED_QUERY_OUTPUT::default(),
            decoder_handle: null_mut(),
            crypto_session_handle: null_mut(),
            device_handle: null_mut(),
        }
    }
}
