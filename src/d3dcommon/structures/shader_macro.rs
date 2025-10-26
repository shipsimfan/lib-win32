use crate::LPCSTR;
use std::ptr::null;

/// Defines a shader macro.
///
/// # Remarks
/// You can use shader macros in your shaders.
///
/// The following shader or effect creation functions take an array of shader macros as an input
/// parameter:
///  - [`D3D10CompileShader`]
///  - [`D3DX10CreateEffectFromFile`]
///  - [`D3DX10PreprocessShaderFromFile`]
///  - [`D3DX11CreateAsyncShaderPreprocessProcessor`]
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D_SHADER_MACRO {
    /// The macro name.
    pub name: LPCSTR,

    /// The macro definition.
    pub definition: LPCSTR,
}

impl Default for D3D_SHADER_MACRO {
    fn default() -> Self {
        D3D_SHADER_MACRO {
            name: null(),
            definition: null(),
        }
    }
}
