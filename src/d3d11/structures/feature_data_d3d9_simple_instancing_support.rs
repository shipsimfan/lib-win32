use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{ID3D11Device, D3D11_FEATURE, D3D11_INPUT_ELEMENT_DESC},
    FALSE, TRUE,
};

/// Describes whether simple instancing is supported.
///
/// # Remarks
/// If the Direct3D API is the Direct3D 11.2 runtime and can support 11.2 features,
/// [`ID3D11Device::check_feature_support`] for [`D3D11_FEATURE::D3D9SimpleInstancingSupport`] will
/// return a `SUCCESS` code when valid parameters are passed. The `simple_instancing_supported`
/// member of [`D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT`] will be set to [`TRUE`] or
/// [`FALSE`].
///
/// Simple instancing means that instancing is supported with the caveat that the
/// `instance_data_step_rate` member of the [`D3D11_INPUT_ELEMENT_DESC`] structure must be equal to
/// 1. This does not change the full instancing support provided by hardware at feature level 9.3
/// and above, and is meant to expose the instancing support that may be available on feature level
/// 9.2 and 9.1 hardware.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    /// Specifies whether the hardware and driver support simple instancing. The runtime sets this
    /// member to [`TRUE`] if the hardware and driver support simple instancing.
    pub simple_instancing_supported: BOOL,
}

impl Default for D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT {
            simple_instancing_supported: 0,
        }
    }
}
