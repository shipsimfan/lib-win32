use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{D3D11_FEATURE_DATA_D3D11_OPTIONS, D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT};
#[allow(unused_imports)]
#[cfg(feature = "d3d11_2")]
use crate::d3d11_2::ID3D11Device2;

/// Shader requires that the graphics driver and hardware support double data type.
pub const D3D_SHADER_REQUIRES_DOUBLES: UINT = 0x00000001;

/// Shader requires an early depth stencil.
pub const D3D_SHADER_REQUIRES_EARLY_DEPTH_STENCIL: UINT = 0x00000002;

/// Shader requires unordered access views (UAVs) at every pipeline stage.
pub const D3D_SHADER_REQUIRES_UAVS_AT_EVERY_STAGE: UINT = 0x00000004;

/// Shader requires 64 UAVs.
pub const D3D_SHADER_REQUIRES_64_UAVS: UINT = 0x00000008;

/// Shader requires the graphics driver and hardware to support minimum precision.
pub const D3D_SHADER_REQUIRES_MINIMUM_PRECISION: UINT = 0x00000010;

/// Shader requires that the graphics driver and hardware support extended doubles instructions.
/// For more info, see the `extended_doubles_shader_instructions` member of
/// [`D3D11_FEATURE_DATA_D3D11_OPTIONS`].
pub const D3D_SHADER_REQUIRES_11_1_DOUBLE_EXTENSIONS: UINT = 0x00000020;

/// Shader requires that the graphics driver and hardware support the msad4 intrinsic function in
/// shaders. For more info, see the `sad4_shader_instructions` member of
/// [`D3D11_FEATURE_DATA_D3D11_OPTIONS`].
pub const D3D_SHADER_REQUIRES_11_1_SHADER_EXTENSIONS: UINT = 0x00000040;

/// Shader requires that the graphics driver and hardware support Direct3D 9 shadow support. For
/// more info, see [`D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT`].
pub const D3D_SHADER_REQUIRES_LEVEL_9_COMPARISON_FILTERING: UINT = 0x00000080;

/// Shader requires that the graphics driver and hardware support tiled resources. For more info,
/// see [`ID3D11Device2::get_resource_tiling`].
pub const D3D_SHADER_REQUIRES_TILED_RESOURCES: UINT = 0x00000100;
