use crate::{
    d3dcommon::{D3D_SHADER_VARIABLE_CLASS, D3D_SHADER_VARIABLE_TYPE},
    LPCSTR, UINT,
};
use std::ptr::null;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11shader::ID3D11ShaderReflectionType;

/// Describes a shader-variable type.
///
/// # Remarks
/// Get a shader-variable-type description by calling [`ID3D11ShaderReflectionType::get_desc`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_SHADER_TYPE_DESC {
    /// A [`D3D_SHADER_VARIABLE_CLASS`]-typed value that identifies the variable class as one of
    /// scalar, vector, matrix, object, and so on.
    pub class: D3D_SHADER_VARIABLE_CLASS,

    /// A [`D3D_SHADER_VARIABLE_TYPE`]-typed value that identifies the variable type.
    pub r#type: D3D_SHADER_VARIABLE_TYPE,

    /// Number of rows in a matrix. Otherwise a numeric type returns 1, any other type returns 0.
    pub rows: UINT,

    /// Number of columns in a matrix. Otherwise a numeric type returns 1, any other type returns
    /// 0.
    pub columns: UINT,

    /// Number of elements in an array; otherwise 0.
    pub elements: UINT,

    /// Number of members in the structure; otherwise 0.
    pub members: UINT,

    /// Offset, in bytes, between the start of the parent structure and this variable. Can be 0 if
    /// not a structure member.
    pub offset: UINT,

    /// Name of the shader-variable type. This member can be [`null`] if it isn't used. This member
    /// supports dynamic shader linkage interface types, which have names.
    pub name: LPCSTR,
}

impl Default for D3D11_SHADER_TYPE_DESC {
    fn default() -> Self {
        D3D11_SHADER_TYPE_DESC {
            class: D3D_SHADER_VARIABLE_CLASS::Scalar,
            r#type: D3D_SHADER_VARIABLE_TYPE::Void,
            rows: 0,
            columns: 0,
            elements: 0,
            members: 0,
            offset: 0,
            name: null(),
        }
    }
}
