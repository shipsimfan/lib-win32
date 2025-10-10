use crate::BOOL;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{ID3D11Device, D3D11_FEATURE},
    FALSE, TRUE,
};

/// Describes Direct3D 9 feature options in the current graphics driver.
///
/// # Remarks
/// You can use the [`D3D11_FEATURE::D3D9Options1`] enumeration value with
/// [`ID3D11Device::check_feature_support`] to query a driver about support for Direct3D 9 feature
/// options rather than making multiple calls to [`ID3D11Device::check_feature_support`] by using
/// [`D3D11_FEATURE::D3D9Options`], [`D3D11_FEATURE::D3D9ShadowSupport`], and
/// [`D3D11_FEATURE::D3D9SimpleInstancingSupport`], which provide identical info about supported
/// Direct3D 9 feature options.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    /// Specifies whether the driver supports the nonpowers-of-2-unconditionally feature. The
    /// runtime sets this member to [`TRUE`] for hardware at Direct3D 10 and higher feature levels.
    /// For hardware at Direct3D 9.3 and lower feature levels, the runtime sets this member to
    /// [`FALSE`] if the hardware and driver support the powers-of-2 (2D textures must have widths
    /// and heights specified as powers of two) feature or the nonpowers-of-2-conditionally
    /// feature.
    pub full_non_pow_2_texture_support: BOOL,

    /// Specifies whether the driver supports the shadowing feature with the comparison-filtering
    /// mode set to less than or equal to. The runtime sets this member to [`TRUE`] for hardware at
    /// Direct3D 10 and higher feature levels. For hardware at Direct3D 9.3 and lower feature
    /// levels, the runtime sets this member to [`TRUE`] only if the hardware and driver support
    /// the shadowing feature; otherwise [`FALSE`].
    pub depth_as_texture_with_less_equal_comparison_filter_supported: BOOL,

    /// Specifies whether the hardware and driver support simple instancing. The runtime sets this
    /// member to [`TRUE`] if the hardware and driver support simple instancing.
    pub simple_instancing_supported: BOOL,

    /// Specifies whether the hardware and driver support setting a single face of a Texture Cube
    /// as a render target while the depth stencil surface that is bound alongside can be a Texture
    /// 2D (as opposed to Texture Cube). The runtime sets this member to [`TRUE`] if the hardware
    /// and driver support this feature; otherwise [`FALSE`].
    ///
    /// If the hardware and driver don't support this feature, the app must match the render target
    /// surface type with the depth stencil surface type. Because hardware at Direct3D 9.3 and
    /// lower feature levels doesn't allow Texture Cube depth surfaces, the only way to render a
    /// scene into a Texture Cube while having depth buffering enabled is to render each Texture
    /// Cube face separately to a Texture 2D render target first (because that can be matched with
    /// a Texture 2D depth), and then copy the results into the Texture Cube. If the hardware and
    /// driver support this feature, the app can just render to the Texture Cube faces directly
    /// while getting depth buffering out of a Texture 2D depth buffer.
    ///
    /// You only need to query this feature from hardware at Direct3D 9.3 and lower feature levels
    /// because hardware at Direct3D 10.0 and higher feature levels allow Texture Cube depth
    /// surfaces.
    pub texture_cube_face_render_target_with_non_cube_depth_stencil_supported: BOOL,
}

impl Default for D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D9_OPTIONS1 {
            full_non_pow_2_texture_support: 0,
            depth_as_texture_with_less_equal_comparison_filter_supported: 0,
            simple_instancing_supported: 0,
            texture_cube_face_render_target_with_non_cube_depth_stencil_supported: 0,
        }
    }
}
