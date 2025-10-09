use crate::{d3d11::D3D11_AUTHENTICATED_QUERY_OUTPUT, HANDLE};
use std::ptr::null_mut;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE;

/// Contains the response to a [`D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE`] query.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    /// A [`D3D11_AUTHENTICATED_QUERY_OUTPUT`] structure that contains a Message Authentication
    /// Code (MAC) and other data.
    pub output: D3D11_AUTHENTICATED_QUERY_OUTPUT,

    /// A handle to the device.
    pub device_handle: HANDLE,
}

impl Default for D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
    fn default() -> Self {
        D3D11_AUTHENTICATED_QUERY_DEVICE_HANDLE_OUTPUT {
            output: D3D11_AUTHENTICATED_QUERY_OUTPUT::default(),
            device_handle: null_mut(),
        }
    }
}
