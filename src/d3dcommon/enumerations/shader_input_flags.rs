// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11shader")]
use crate::d3d11shader::D3D11_SHADER_INPUT_BIND_DESC;

/// Values that identify shader-input options.
///
/// # Remarks
/// [`D3D_SHADER_INPUT_FLAGS`]-typed values are specified in the uFlags member of the
/// [`D3D11_SHADER_INPUT_BIND_DESC`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D_SHADER_INPUT_FLAGS {
    /// Assign a shader input to a register based on the register assignment in the HLSL code
    /// (instead of letting the compiler choose the register).
    UserPacked = 0x1,

    /// Use a comparison sampler, which uses the `SampleCmp` (DirectX HLSL Texture Object) and
    /// `SampleCmpLevelZero` (DirectX HLSL Texture Object) sampling functions.
    ComparisonSampler = 0x2,

    /// A 2-bit value for encoding texture components.
    TextureComponent0 = 0x4,

    /// A 2-bit value for encoding texture components.
    TextureComponent1 = 0x8,

    /// A 2-bit value for encoding texture components.
    TextureComponents = 0xC,

    /// This value is reserved.
    Unused = 0x10,

    /// Assign a shader input to a register based on the register assignment in the HLSL code
    /// (instead of letting the compiler choose the register).
    D3D10UserPacked,

    /// Use a comparison sampler, which uses the `SampleCmp` (DirectX HLSL Texture Object) and
    /// `SampleCmpLevelZero` (DirectX HLSL Texture Object) sampling functions.
    D3D10ComparisonSampler,

    /// A 2-bit value for encoding texture components.
    D3D10TextureComponent0,

    /// A 2-bit value for encoding texture components.
    D3D10TextureComponent1,

    /// A 2-bit value for encoding texture components.
    D3D10TextureComponents,

    /// Forces the enumeration to compile to 32 bits. This value is not used directly by titles.
    ForceDword = 0x7FFFFFFF,
}
