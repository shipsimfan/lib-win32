use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_SHADER_MIN_PRECISION_SUPPORT;

/// Describes precision support options for shaders in the current graphics driver.
///
/// # Remarks
/// For hardware at Direct3D 10 and higher feature levels, the runtime sets both members
/// identically. For hardware at Direct3D 9.3 and lower feature levels, the runtime can set a lower
/// precision support in the `pixel_shader_min_precision` member than the
/// `all_other_shader_stages_min_precision` member; for 9.3 and lower, all other shader stages
/// represent only the vertex shader.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    /// A combination of [`D3D11_SHADER_MIN_PRECISION_SUPPORT`]-typed values that are combined by
    /// using a bitwise OR operation. The resulting value specifies minimum precision levels that
    /// the driver supports for the pixel shader. A value of zero indicates that the driver
    /// supports only full 32-bit precision for the pixel shader.
    pub pixel_shader_min_precision: UINT,

    /// A combination of [`D3D11_SHADER_MIN_PRECISION_SUPPORT`]-typed values that are combined by
    /// using a bitwise OR operation. The resulting value specifies minimum precision levels that
    /// the driver supports for all other shader stages. A value of zero indicates that the driver
    /// supports only full 32-bit precision for all other shader stages.
    pub all_other_shader_stages_min_precision: UINT,
}

impl Default for D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
    fn default() -> Self {
        D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT {
            pixel_shader_min_precision: 0,
            all_other_shader_stages_min_precision: 0,
        }
    }
}
