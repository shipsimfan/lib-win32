use crate::d3d11::D3D11_SHARED_RESOURCE_TIER;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_FEATURE;

/// Describes the level of support for shared resources in the current graphics driver.
///
/// # Remarks
/// Use this structure with the [`D3D11_FEATURE::D3D11Options5`] member of [`D3D11_FEATURE`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    /// The level of support for shared resources in the current graphics driver.
    pub shared_resource_tier: D3D11_SHARED_RESOURCE_TIER,
}

impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D11_OPTIONS5 {
            shared_resource_tier: D3D11_SHARED_RESOURCE_TIER::_0,
        }
    }
}
