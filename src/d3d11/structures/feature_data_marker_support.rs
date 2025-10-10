use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{ID3D11Device, D3D11_FEATURE},
    FALSE, TRUE,
};

/// Describes whether a GPU profiling technique is supported.
///
/// # Remarks
/// If the Direct3D API is the Direct3D 11.2 runtime and can support 11.2 features,
/// [`ID3D11Device::check_feature_support`] for [`D3D11_FEATURE::MarkerSupport`] will return a
/// `SUCCESS` code when valid parameters are passed. The `profile` member of
/// [`D3D11_FEATURE_DATA_MARKER_SUPPORT`] will be set to [`TRUE`] or [`FALSE`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_MARKER_SUPPORT {
    /// Specifies whether the hardware and driver support a GPU profiling technique that can be
    /// used with development tools. The runtime sets this member to [`TRUE`] if the hardware and
    /// driver support data marking.
    pub profile: BOOL,
}

impl Default for D3D11_FEATURE_DATA_MARKER_SUPPORT {
    fn default() -> Self {
        D3D11_FEATURE_DATA_MARKER_SUPPORT { profile: 0 }
    }
}
