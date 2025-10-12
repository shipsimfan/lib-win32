use std::ptr::null;

use crate::{
    d3dcommon::{D3D_RESOURCE_RETURN_TYPE, D3D_SHADER_INPUT_TYPE, D3D_SRV_DIMENSION},
    LPCSTR, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11shader::ID3D11ShaderReflection, d3dcommon::D3D_SHADER_INPUT_FLAGS};

/// Describes how a shader resource is bound to a shader input.
///
/// # Remarks
/// Get a shader-input-signature description by calling
/// [`ID3D11ShaderReflection::get_resource_binding_desc`] or
/// [`ID3D11ShaderReflection::get_resource_binding_desc_by_name`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_SHADER_INPUT_BIND_DESC {
    /// Name of the shader resource.
    pub name: LPCSTR,

    /// A [`D3D_SHADER_INPUT_TYPE`]-typed value that identifies the type of data in the resource.
    pub r#type: D3D_SHADER_INPUT_TYPE,

    /// Starting bind point.
    pub bind_point: UINT,

    /// Number of contiguous bind points for arrays.
    pub bind_count: UINT,

    /// A combination of [`D3D_SHADER_INPUT_FLAGS`]-typed values for shader input-parameter
    /// options.
    pub flags: UINT,

    /// If the input is a texture, the [`D3D_RESOURCE_RETURN_TYPE`]-typed value that identifies the
    /// return type.
    pub return_type: D3D_RESOURCE_RETURN_TYPE,

    /// A [`D3D_SRV_DIMENSION`]-typed value that identifies the dimensions of the bound resource.
    pub dimension: D3D_SRV_DIMENSION,

    /// The number of samples for a multisampled texture; when a texture isn't multisampled, the
    /// value is set to -1 (0xFFFFFFFF).
    pub num_samples: UINT,
}

impl Default for D3D11_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        D3D11_SHADER_INPUT_BIND_DESC {
            name: null(),
            r#type: D3D_SHADER_INPUT_TYPE::CBuffer,
            bind_point: 0,
            bind_count: 0,
            flags: 0,
            return_type: D3D_RESOURCE_RETURN_TYPE::UNorm,
            dimension: D3D_SRV_DIMENSION::Unknown,
            num_samples: 0,
        }
    }
}
