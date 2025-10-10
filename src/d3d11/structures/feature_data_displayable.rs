use crate::{d3d11::D3D11_SHARED_RESOURCE_TIER, BOOL};

// rustdoc imports
#[allow(unused_imports)]
use crate::{FALSE, TRUE};

/// Describes the level of displayable surfaces supported in the current graphics driver.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_DISPLAYABLE {
    /// [`TRUE`] if the driver supports displayable surfaces; otherwise, [`FALSE`].
    pub displayable_texture: BOOL,

    /// The level of support for shared resources in the current graphics driver.
    pub shared_resource_tier: D3D11_SHARED_RESOURCE_TIER,
}

impl Default for D3D11_FEATURE_DATA_DISPLAYABLE {
    fn default() -> Self {
        D3D11_FEATURE_DATA_DISPLAYABLE {
            displayable_texture: 0,
            shared_resource_tier: D3D11_SHARED_RESOURCE_TIER::_0,
        }
    }
}
