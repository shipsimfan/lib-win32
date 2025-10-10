use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_FEATURE;

/// Describes Direct3D 11.4 feature options in the current graphics driver.
///
/// # Remarks
/// Use this structure with the [`D3D11_FEATURE::D3D11Options4`] member of [`D3D11_FEATURE`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    /// Specifies a [`BOOL`] that determines if NV12 textures can be shared across processes and
    /// D3D devices.
    pub extended_nv12_shared_texture_supported: BOOL,
}

impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D11_OPTIONS4 {
            extended_nv12_shared_texture_supported: 0,
        }
    }
}
