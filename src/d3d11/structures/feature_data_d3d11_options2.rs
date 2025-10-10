use crate::{
    d3d11::{D3D11_CONSERVATIVE_RASTERIZATION_TIER, D3D11_TILED_RESOURCES_TIER},
    BOOL,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{D3D11_CPU_ACCESS_FLAG, D3D11_USAGE},
    d3d11_3::D3D11_TEXTURE_LAYOUT,
    TRUE,
};

/// Describes Direct3D 11.3 feature options in the current graphics driver.
///
/// # Remarks
/// If `map_on_default_textures` is [`TRUE`], applications may create textures using
/// [`D3D11_USAGE::Default`] in combination with non-zero a [`D3D11_CPU_ACCESS_FLAG`] value. For
/// performance reasons it is typically undesirable to create a default texture with CPU access
/// flags unless the `unified_memory_architecture` option is [`TRUE`], or CPU / GPU usage of the
/// texture is tightly interleaved.
///
/// Default textures may not be in a mapped state while either bound to the pipeline to referenced
/// by an operation issued to a context. Default textures may not be mapped by a deferred context.
/// Default textures may not be created shareable.
///
/// See [`D3D11_TEXTURE_LAYOUT`] for texture swizzle options and restrictions.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    /// Specifies whether the hardware and driver support `ps_specified_stencil_ref_supported`. The
    /// runtime sets this member to [`TRUE`] if the hardware and driver support this option.
    pub ps_specified_stencil_ref_supported: BOOL,

    /// Specifies whether the hardware and driver support `typed_uav_load_additional_formats`. The
    /// runtime sets this member to [`TRUE`] if the hardware and driver support this option.
    pub typed_uav_load_additional_formats: BOOL,

    /// Specifies whether the hardware and driver support ROVs. The runtime sets this member to
    /// [`TRUE`] if the hardware and driver support this option.
    pub rovs_supported: BOOL,

    /// Specifies whether the hardware and driver support conservative rasterization. The runtime
    /// sets this member to a [`D3D11_CONSERVATIVE_RASTERIZATION_TIER`]-typed value that indicates
    /// if the hardware and driver support conservative rasterization and at what tier level.
    pub conservative_rasterization_tier: D3D11_CONSERVATIVE_RASTERIZATION_TIER,

    /// Specifies whether the hardware and driver support tiled resources. The runtime sets this
    /// member to a [`D3D11_TILED_RESOURCES_TIER`]-typed value that indicates if the hardware and
    /// driver support tiled resources and at what tier level.
    pub tiled_resources_tier: D3D11_TILED_RESOURCES_TIER,

    /// Specifies whether the hardware and driver support mapping on default textures. The runtime
    /// sets this member to [`TRUE`] if the hardware and driver support this option.
    pub map_on_default_textures: BOOL,

    /// Specifies whether the hardware and driver support standard swizzle. The runtime sets this
    /// member to [`TRUE`] if the hardware and driver support this option.
    pub standard_swizzle: BOOL,

    /// Specifies whether the hardware and driver support Unified Memory Architecture. The runtime
    /// sets this member to [`TRUE`] if the hardware and driver support this option.
    pub unified_memory_architecture: BOOL,
}

impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D11_OPTIONS2 {
            ps_specified_stencil_ref_supported: 0,
            typed_uav_load_additional_formats: 0,
            rovs_supported: 0,
            conservative_rasterization_tier: D3D11_CONSERVATIVE_RASTERIZATION_TIER::NotSupported,
            tiled_resources_tier: D3D11_TILED_RESOURCES_TIER::NotSupported,
            map_on_default_textures: 0,
            standard_swizzle: 0,
            unified_memory_architecture: 0,
        }
    }
}
