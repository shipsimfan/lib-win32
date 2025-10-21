// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11shader")]
use crate::d3d11shader::D3D11_SHADER_BUFFER_DESC;

/// Values that identify the intended use of a constant-data buffer.
///
/// # Remarks
/// [`D3D_SHADER_CBUFFER_FLAGS`]-typed values are specified in the `flags` member of the
/// [`D3D11_SHADER_BUFFER_DESC`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D_SHADER_CBUFFER_FLAGS {
    /// Bind the constant buffer to an input slot defined in HLSL code (instead of letting the
    /// compiler choose the input slot).
    UserPacked = 1,

    /// Bind the constant buffer to an input slot defined in HLSL code (instead of letting the
    /// compiler choose the input slot).
    D3D10UserPacked,

    /// This value is not used by a programmer; it exists to force the enumeration to compile to 32
    /// bits.
    ForceDword = 0x7FFFFFFF,
}
