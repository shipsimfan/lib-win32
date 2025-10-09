use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3dcommon::D3D_FEATURE_LEVEL, FALSE, TRUE};

/// Describes compute shader and raw and structured buffer support in the current graphics driver.
///
/// # Remarks
/// Direct3D 11 devices ([`D3D_FEATURE_LEVEL::_11_0`]) are required to support Compute Shader model
/// 5.0. Direct3D 10.x devices ([`D3D_FEATURE_LEVEL::_10_0`], [`D3D_FEATURE_LEVEL::_10_1`]) can
/// optionally support Compute Shader model 4.0 or 4.1.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    /// [`TRUE`] if compute shaders and raw and structured buffers are supported; otherwise
    /// [`FALSE`].
    pub compute_shaders_plus_raw_and_structured_buffers_via_shader_4_x: BOOL,
}

impl Default for D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS {
            compute_shaders_plus_raw_and_structured_buffers_via_shader_4_x: 0,
        }
    }
}
