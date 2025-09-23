use crate::{d3d11::D3D11_AUTHENTICATED_CONFIGURE_INPUT, HANDLE};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION;

/// Contains input data for a [`D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION`] command.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_CONFIGURE_CRYPTO_SESSION_INPUT {
    /// A [`D3D11_AUTHENTICATED_CONFIGURE_INPUT`] structure that contains the command GUID and
    /// other data.
    pub parameters: D3D11_AUTHENTICATED_CONFIGURE_INPUT,

    /// A handle to the decoder device. Get this from [`ID3D11VideoDecoder::get_driver_handle`].
    pub decoder_handle: HANDLE,

    /// A handle to the cryptographic session. Get this from
    /// [`ID3D11CryptoSession::get_crypto_session_handle`].
    pub crypto_session_handle: HANDLE,

    /// A handle to the Direct3D device. Get this from
    /// [`D3D11VideoContext::query_authenticated_channel`] using
    /// [`D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE`].
    pub device_handle: HANDLE,
}
