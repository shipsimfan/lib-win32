use crate::{dxgi::DXGI_FORMAT, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_FORMAT_SUPPORT;

/// Describes which resources are supported by the current graphics driver for a given format.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    /// [`DXGI_FORMAT`] to return information on.
    pub in_format: DXGI_FORMAT,

    /// Combination of [`D3D11_FORMAT_SUPPORT`] flags indicating which resources are supported.
    pub out_format_support: UINT,
}

impl Default for D3D11_FEATURE_DATA_FORMAT_SUPPORT {
    fn default() -> Self {
        D3D11_FEATURE_DATA_FORMAT_SUPPORT {
            in_format: DXGI_FORMAT::Unknown,
            out_format_support: 0,
        }
    }
}
