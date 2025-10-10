// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{
    D3D11_FEATURE_DATA_ARCHITECTURE_INFO, D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS,
    D3D11_FEATURE_DATA_D3D11_OPTIONS, D3D11_FEATURE_DATA_D3D11_OPTIONS1,
    D3D11_FEATURE_DATA_D3D9_OPTIONS, D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT,
    D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT, D3D11_FEATURE_DATA_DOUBLES,
    D3D11_FEATURE_DATA_FORMAT_SUPPORT, D3D11_FEATURE_DATA_FORMAT_SUPPORT2,
    D3D11_FEATURE_DATA_MARKER_SUPPORT, D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT,
    D3D11_FEATURE_DATA_THREADING,
};

/// Direct3D 11 feature options.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_FEATURE {
    /// The driver supports multithreading. Refer to [`D3D11_FEATURE_DATA_THREADING`].
    Threading,

    /// Supports the use of the double-precision shaders in HLSL. Refer to
    /// [`D3D11_FEATURE_DATA_DOUBLES`].
    Doubles,

    /// Supports the formats in D3D11FORMAT_Support. Refer to
    /// [`D3D11_FEATURE_DATA_FORMAT_SUPPORT`].
    FormatSupport,

    /// Supports the formats in D3D11FORMAT_Support2. Refer to
    /// [`D3D11_FEATURE_DATA_FORMAT_SUPPORT2`].
    FormatSupport2,

    /// Supports compute shaders and raw and structured buffers. Refer to
    /// [`D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS`].
    D3D10XHardwareOptions,

    /// Supports Direct3D 11.1 feature options. Refer to [`D3D11_FEATURE_DATA_D3D11_OPTIONS`].
    D3D11Options,

    /// Supports specific adapter architecture. Refer to [`D3D11_FEATURE_DATA_ARCHITECTURE_INFO`].
    ArchitectureInfo,

    /// Supports Direct3D 9 feature options. Refer to [`D3D11_FEATURE_DATA_D3D9_OPTIONS`].
    D3D9Options,

    /// Supports minimum precision of shaders. Refer to
    /// [`D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT`].
    ShaderMinPrecisionSupport,

    /// Supports Direct3D 9 shadowing feature. Refer to [`D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT`].
    D3D9ShadowSupport,

    /// Supports Direct3D 11.2 feature options. Refer to [`D3D11_FEATURE_DATA_D3D11_OPTIONS1`].
    D3D11Options1,

    /// Supports Direct3D 11.2 instancing options. Refer to
    /// [`D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT`].
    D3D9SimpleInstancingSupport,

    /// Supports Direct3D 11.2 marker options. Refer to [`D3D11_FEATURE_DATA_MARKER_SUPPORT`].
    MarkerSupport,

    /// Supports Direct3D 9 feature options, which includes the Direct3D 9 shadowing feature and
    /// instancing support. Refer to [`D3D11_FEATURE_DATA_D3D9_OPTIONS1`].
    D3D9Options1,

    /// Supports Direct3D 11.3 conservative rasterization feature options. Refer to
    /// [`D3D11_FEATURE_DATA_D3D11_OPTIONS2`].
    D3D11Options2,

    /// Supports Direct3D 11.4 conservative rasterization feature options. Refer to
    /// [`D3D11_FEATURE_DATA_D3D11_OPTIONS3`].
    D3D11Options3,

    /// Supports GPU virtual addresses. Refer to
    /// [`D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT`].
    GpuVirtualAddressSupport,

    /// Supports a single boolean for NV12 shared textures. Refer to
    /// [`D3D11_FEATURE_DATA_D3D11_OPTIONS4`].
    D3D11Options4,

    /// Supports shader cache, described in [`D3D11_FEATURE_DATA_SHADER_CACHE`].
    ShaderCACHE,

    /// Supports a [`D3D11SHARED_RESOURCE_TIER`] to indicate the level of support for shared
    /// resources in the current graphics driver. Refer to [`D3D11_FEATURE_DATA_D3D11_OPTIONS5`].
    D3D11Options5,

    /// Supports displayable surfaces, described in [`D3D11_FEATURE_DATA_DISPLAYABLE`].
    Displayable,
}
