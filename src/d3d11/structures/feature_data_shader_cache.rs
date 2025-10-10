use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, D3D11_FEATURE};

/// Describes the level of shader caching supported in the current graphics driver.
///
/// # Remarks
/// Use this structure with [`ID3D11Device::check_feature_support`] to determine the level of
/// support offered for the optional shader-caching features.
///
/// See the enumeration constant [`D3D11_FEATURE::ShaderCache`] in the [`D3D11_FEATURE`]
/// enumeration.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_SHADER_CACHE {
    /// Indicates the level of caching supported.
    pub support_flags: UINT,
}

impl Default for D3D11_FEATURE_DATA_SHADER_CACHE {
    fn default() -> Self {
        D3D11_FEATURE_DATA_SHADER_CACHE { support_flags: 0 }
    }
}
