use crate::{d3dcommon::D3D_CBUFFER_TYPE, LPCSTR, UINT};
use std::ptr::null;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11shader::ID3D11ShaderReflectionConstantBuffer, d3dcommon::D3D_SHADER_CBUFFER_FLAGS,
};

/// Describes a shader constant-buffer.
///
/// # Remarks
/// Constants are supplied to shaders in a shader-constant buffer. Get the description of a
/// shader-constant-buffer by calling [`ID3D11ShaderReflectionConstantBuffer::get_desc`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_SHADER_BUFFER_DESC {
    /// The name of the buffer.
    pub name: LPCSTR,

    /// A [`D3D_CBUFFER_TYPE`]-typed value that indicates the intended use of the constant data.
    pub r#type: D3D_CBUFFER_TYPE,

    /// The number of unique variables.
    pub variables: UINT,

    /// Buffer size (in bytes).
    pub size: UINT,

    /// A combination of [`D3D_SHADER_CBUFFER_FLAGS`]-typed values that are combined by using a
    /// bitwise OR operation. The resulting value specifies properties for the shader
    /// constant-buffer.
    pub flags: UINT,
}

impl Default for D3D11_SHADER_BUFFER_DESC {
    fn default() -> Self {
        D3D11_SHADER_BUFFER_DESC {
            name: null(),
            r#type: D3D_CBUFFER_TYPE::CBuffer,
            variables: 0,
            size: 0,
            flags: 0,
        }
    }
}
