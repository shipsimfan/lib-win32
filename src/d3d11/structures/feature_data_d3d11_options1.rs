use crate::{d3d11::D3D11_TILED_RESOURCES_TIER, BOOL};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{
        ID3D11Buffer, ID3D11Device, ID3D11DeviceContext, D3D11_BUFFER_DESC, D3D11_CPU_ACCESS_FLAG,
        D3D11_FILTER, D3D11_USAGE,
    },
    d3d11_1::ID3D11DeviceContext1,
    d3dcommon::D3D_FEATURE_LEVEL,
    TRUE,
};

/// Describes Direct3D 11.2 feature options in the current graphics driver.
///
/// # Remarks
/// If the Direct3D API is the Direct3D 11.2 runtime and can support 11.2 features,
/// [`ID3D11Device::check_feature_support`] for [`D3D11_FEATURE::D3D11Options1`] will return a
/// `SUCCESS` code when valid parameters are passed. The members of
/// [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`] will be set appropriately based on the system's graphics
/// hardware and graphics driver.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    /// Specifies whether the hardware and driver support tiled resources. The runtime sets this
    /// member to a [`D3D11_TILED_RESOURCES_TIER`]-typed value that indicates if the hardware and
    /// driver support tiled resources and at what tier level.
    pub tiled_resources_tier: D3D11_TILED_RESOURCES_TIER,

    /// Specifies whether the hardware and driver support the filtering options ([`D3D11_FILTER`])
    /// of comparing the result to the minimum or maximum value during texture sampling. The
    /// runtime sets this member to [`TRUE`] if the hardware and driver support these filtering
    /// options.
    pub min_max_filtering: BOOL,

    /// Specifies whether the hardware and driver also support the
    /// [`ID3D11DeviceContext1::clear_view`] method on depth formats.
    pub clear_view_also_supports_depth_only_formats: BOOL,

    /// Specifies support for creating [`ID3D11Buffer`] resources that can be passed to the
    /// [`ID3D11DeviceContext::map`] and [`ID3D11DeviceContext::unmap`] methods. This means that
    /// the `cpu_access_flags` member of the [`D3D11_BUFFER_DESC`] structure may be set with the
    /// desired [`D3D11_CPU_ACCESS_FLAG`] elements when the `usage` member of [`D3D11_BUFFER_DESC`]
    /// is set to [`D3D11_USAGE::Default`]. The runtime sets this member to [`TRUE`] if the
    /// hardware is capable of at least [`D3D_FEATURE_LEVEL::_11_0`] and the graphics device driver
    /// supports mappable default buffers.
    pub map_on_default_buffers: BOOL,
}

impl Default for D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
    fn default() -> Self {
        D3D11_FEATURE_DATA_D3D11_OPTIONS1 {
            tiled_resources_tier: D3D11_TILED_RESOURCES_TIER::NotSupported,
            min_max_filtering: 0,
            clear_view_also_supports_depth_only_formats: 0,
            map_on_default_buffers: 0,
        }
    }
}
