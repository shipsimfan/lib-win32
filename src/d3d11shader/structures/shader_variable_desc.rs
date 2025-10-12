use crate::{LPCSTR, LPVOID, UINT};
use std::ptr::{null, null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11shader::ID3D11ShaderReflectionVariable, d3dcommon::D3D_SHADER_VARIABLE_FLAGS};

/// Describes a shader variable.
///
/// # Remarks
/// Get a shader-variable description using reflection by calling
/// [`ID3D11ShaderReflectionVariable::get_desc`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_SHADER_VARIABLE_DESC {
    /// The variable name.
    pub name: LPCSTR,

    /// Offset from the start of the parent structure to the beginning of the variable.
    pub start_offset: UINT,

    /// Size of the variable (in bytes).
    pub size: UINT,

    /// A combination of [`D3D_SHADER_VARIABLE_FLAGS`]-typed values that are combined by using a
    /// bitwise OR operation. The resulting value identifies shader-variable properties.
    pub flags: UINT,

    /// The default value for initializing the variable.
    pub default_value: LPVOID,

    /// Offset from the start of the variable to the beginning of the texture.
    pub start_texture: UINT,

    /// The size of the texture, in bytes.
    pub texture_size: UINT,

    /// Offset from the start of the variable to the beginning of the sampler.
    pub start_sampler: UINT,

    /// The size of the sampler, in bytes.
    pub sampler_size: UINT,
}

impl Default for D3D11_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        D3D11_SHADER_VARIABLE_DESC {
            name: null(),
            start_offset: 0,
            size: 0,
            flags: 0,
            default_value: null_mut(),
            start_texture: 0,
            texture_size: 0,
            start_sampler: 0,
            sampler_size: 0,
        }
    }
}
