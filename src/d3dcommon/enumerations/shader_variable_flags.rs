// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11shader")]
use crate::d3d11shader::{ID3D11ShaderReflectionVariable, D3D11_SHADER_VARIABLE_DESC};

/// Values that identify information about a shader variable.
///
/// # Remarks
/// A call to the [`ID3D11ShaderReflectionVariable::get_desc`] method returns
/// [`D3D_SHADER_VARIABLE_FLAGS`] values in the `flags` member of a [`D3D11_SHADER_VARIABLE_DESC`]
/// structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D_SHADER_VARIABLE_FLAGS {
    /// Indicates that the registers assigned to this shader variable were explicitly declared in
    /// shader code (instead of automatically assigned by the compiler).
    UserPacked = 1,

    /// Indicates that this variable is used by this shader. This value confirms that a particular
    /// shader variable (which can be common to many different shaders) is indeed used by a
    /// particular shader.
    Used = 2,

    /// Indicates that this variable is an interface.
    InterfacePointer = 4,

    /// Indicates that this variable is a parameter of an interface.
    InterfaceParameter = 8,

    /// Indicates that the registers assigned to this shader variable were explicitly declared in
    /// shader code (instead of automatically assigned by the compiler).
    D3D10UserPacked,

    /// Indicates that this variable is used by this shader. This value confirms that a particular
    /// shader variable (which can be common to many different shaders) is indeed used by a
    /// particular shader.
    D3D10Used,

    /// Indicates that this variable is an interface.
    D3D11InterfacePointer,

    /// Indicates that this variable is a parameter of an interface.
    D3D11InterfaceParameter,

    /// This value is not used by a programmer; it exists to force the enumeration to compile to 32
    /// bits.
    ForceDword = 0x7FFFFFFF,
}
