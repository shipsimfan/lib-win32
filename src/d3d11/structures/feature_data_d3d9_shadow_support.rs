use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::D3D11_COMPARISON_FUNC, FALSE, TRUE};

/// Describes Direct3D 9 shadow support in the current graphics driver.
///
/// # Remarks
/// Shadows are an important element in realistic 3D scenes. You can use the shadow buffer
/// technique to render shadows. The basic principle of the technique is to use a depth buffer to
/// store the scene depth info from the perspective of the light source, and then compare each
/// point rendered in the scene with that buffer to determine if it is in shadow.
///
/// To render objects into the scene with shadows on them, you create sampler state objects with
/// comparison filtering set and the comparison mode to [`D3D11_COMPARISON_FUNC::LessEqual`]. You
/// can also set `border_color` addressing on this depth sampler, even though `border_color` isn't
/// typically allowed on feature levels 9.1 and 9.2. By using the border color and picking 0.0 or
/// 1.0 as the border color value, you can control whether the regions off the edge of the shadow
/// map appear to be always in shadow or never in shadow respectively. You can control the shadow
/// filter quality by the Mag and Min filter settings in the comparison sampler. Point sampling
/// will produce shadows with non-anti-aliased edges. Linear filter sampler settings will result in
/// higher quality shadow edges, but might affect performance on some power-optimized devices.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    /// Specifies whether the driver supports the shadowing feature with the comparison-filtering
    /// mode set to less than or equal to. The runtime sets this member to [`TRUE`] for hardware at
    /// Direct3D 10 and higher feature levels. For hardware at Direct3D 9.3 and lower feature
    /// levels, the runtime sets this member to [`TRUE`] only if the hardware and driver support
    /// the shadowing feature; otherwise [`FALSE`].
    pub supports_depth_as_texture_with_less_equal_comparison_filter: BOOL,
}

impl Default for D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT {
            supports_depth_as_texture_with_less_equal_comparison_filter: 0,
        }
    }
}
