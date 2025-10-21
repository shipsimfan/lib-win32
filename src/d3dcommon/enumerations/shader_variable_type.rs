// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11shader")]
use crate::d3d11shader::{ID3D11ShaderReflectionType, D3D11_SHADER_TYPE_DESC};

/// Values that identify various data, texture, and buffer types that can be assigned to a shader
/// variable.
///
/// # Remarks
/// A call to the [`ID3D11ShaderReflectionType::get_desc`] method returns a
/// [`D3D_SHADER_VARIABLE_TYPE`] value in the Type member of a [`D3D11_SHADER_TYPE_DESC`]
/// structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D_SHADER_VARIABLE_TYPE {
    /// The variable is a void pointer.
    Void = 0,

    /// The variable is a boolean.
    Bool = 1,

    /// The variable is an integer.
    Int = 2,

    /// The variable is a floating-point number.
    Float = 3,

    /// The variable is a string.
    String = 4,

    /// The variable is a texture.
    Texture = 5,

    /// The variable is a 1D texture.
    Texture1D = 6,

    /// The variable is a 2D texture.
    Texture2D = 7,

    /// The variable is a 3D texture.
    Texture3D = 8,

    /// The variable is a texture cube.
    TextureCube = 9,

    /// The variable is a sampler.
    Sampler = 10,

    /// The variable is a 1D sampler.
    Sampler1D = 11,

    /// The variable is a 2D sampler.
    Sampler2D = 12,

    /// The variable is a 3D sampler.
    Sampler3D = 13,

    /// The variable is a cube sampler.
    SamplerCube = 14,

    /// The variable is a pixel shader.
    PixelShader = 15,

    /// The variable is a vertex shader.
    VertexShader = 16,

    /// The variable is a pixel fragment.
    PixelFragment = 17,

    /// The variable is a vertex fragment.
    VertexFragment = 18,

    /// The variable is an unsigned integer.
    UInt = 19,

    /// The variable is an 8-bit unsigned integer.
    UInt8 = 20,

    /// The variable is a geometry shader.
    GeometryShader = 21,

    /// The variable is a rasterizer-state object.
    Rasterizer = 22,

    /// The variable is a depth-stencil-state object.
    DepthStencil = 23,

    /// The variable is a blend-state object.
    Blend = 24,

    /// The variable is a buffer.
    Buffer = 25,

    /// The variable is a constant buffer.
    CBuffer = 26,

    /// The variable is a texture buffer.
    TBuffer = 27,

    /// The variable is a 1D-texture array.
    Texture1DArray = 28,

    /// The variable is a 2D-texture array.
    Texture2DArray = 29,

    /// The variable is a render-target view.
    RenderTargetView = 30,

    /// The variable is a depth-stencil view.
    DepthStencilView = 31,

    /// The variable is a 2D-multisampled texture.
    Texture2DMS = 32,

    /// The variable is a 2D-multisampled-texture array.
    Texture2DMSArray = 33,

    /// The variable is a texture-cube array.
    TextureCubeArray = 34,

    /// The variable holds a compiled hull-shader binary.
    HullShader = 35,

    /// The variable holds a compiled domain-shader binary.
    DomainShader = 36,

    /// The variable is an interface.
    InterfacePointer = 37,

    /// The variable holds a compiled compute-shader binary.
    ComputeShader = 38,

    /// The variable is a double precision (64-bit) floating-point number.
    Double = 39,

    /// The variable is a 1D read-and-write texture.
    RwTexture1D = 40,

    /// The variable is an array of 1D read-and-write textures.
    RwTexture1DArray = 41,

    /// The variable is a 2D read-and-write texture.
    RwTexture2D = 42,

    /// The variable is an array of 2D read-and-write textures.
    RwTexture2DArray = 43,

    /// The variable is a 3D read-and-write texture.
    RwTexture3D = 44,

    /// The variable is a read-and-write buffer.
    RwBuffer = 45,

    /// The variable is a byte-address buffer.
    ByteAddressBuffer = 46,

    /// The variable is a read-and-write byte-address buffer.
    RwByteAddressBuffer = 47,

    /// The variable is a structured buffer.
    StructuredBuffer = 48,

    /// The variable is a read-and-write structured buffer.
    RwStructuredBuffer = 49,

    /// The variable is an append structured buffer.
    AppendStructuredBuffer = 50,

    /// The variable is a consume structured buffer.
    ConsumeStructuredBuffer = 51,

    /// The variable is an 8-byte Float.
    Min8Float = 52,

    /// The variable is a 10-byte Float.
    Min10Float = 53,

    /// The variable is a 16-byte Float.
    Min16Float = 54,

    /// The variable is a 12-byte Int.
    Min12Int = 55,

    /// The variable is a 16-byte Int.
    Min16Int = 56,

    /// The variable is a 16-byte Int.
    Min16UInt = 57,

    #[allow(missing_docs)]
    D3D10Void,

    #[allow(missing_docs)]
    D3D10Bool,

    #[allow(missing_docs)]
    D3D10Int,

    #[allow(missing_docs)]
    D3D10Float,

    #[allow(missing_docs)]
    D3D10String,

    #[allow(missing_docs)]
    D3D10Texture,

    #[allow(missing_docs)]
    D3D10Texture1D,

    #[allow(missing_docs)]
    D3D10Texture2D,

    #[allow(missing_docs)]
    D3D10Texture3D,

    #[allow(missing_docs)]
    D3D10TextureCube,

    #[allow(missing_docs)]
    D3D10Sampler,

    #[allow(missing_docs)]
    D3D10Sampler1D,

    #[allow(missing_docs)]
    D3D10Sampler2D,

    #[allow(missing_docs)]
    D3D10Sampler3D,

    #[allow(missing_docs)]
    D3D10SamplerCube,

    #[allow(missing_docs)]
    D3D10PixelShader,

    #[allow(missing_docs)]
    D3D10VertexShader,

    #[allow(missing_docs)]
    D3D10PixelFragment,

    #[allow(missing_docs)]
    D3D10VertexFragment,

    #[allow(missing_docs)]
    D3D10UInt,

    #[allow(missing_docs)]
    D3D10UInt8,

    #[allow(missing_docs)]
    D3D10GeometryShader,

    #[allow(missing_docs)]
    D3D10Rasterizer,

    #[allow(missing_docs)]
    D3D10DepthStencil,

    #[allow(missing_docs)]
    D3D10Blend,

    #[allow(missing_docs)]
    D3D10Buffer,

    #[allow(missing_docs)]
    D3D10CBuffer,

    #[allow(missing_docs)]
    D3D10TBuffer,

    #[allow(missing_docs)]
    D3D10Texture1DArray,

    #[allow(missing_docs)]
    D3D10Texture2DArray,

    #[allow(missing_docs)]
    D3D10RenderTargetView,

    #[allow(missing_docs)]
    D3D10DepthStencilView,

    #[allow(missing_docs)]
    D3D10Texture2DMS,

    #[allow(missing_docs)]
    D3D10Texture2DMSArray,

    #[allow(missing_docs)]
    D3D10TextureCubeArray,

    #[allow(missing_docs)]
    D3D11HullShader,

    #[allow(missing_docs)]
    D3D11DomainShader,

    #[allow(missing_docs)]
    D3D11InterfacePointer,

    #[allow(missing_docs)]
    D3D11ComputeShader,

    #[allow(missing_docs)]
    D3D11Double,

    #[allow(missing_docs)]
    D3D11RwTexture1D,

    #[allow(missing_docs)]
    D3D11RwTexture1DArray,

    #[allow(missing_docs)]
    D3D11RwTexture2D,

    #[allow(missing_docs)]
    D3D11RwTexture2DArray,

    #[allow(missing_docs)]
    D3D11RwTexture3D,

    #[allow(missing_docs)]
    D3D11RwBuffer,

    #[allow(missing_docs)]
    D3D11ByteAddressBuffer,

    #[allow(missing_docs)]
    D3D11RwByteAddressBuffer,

    #[allow(missing_docs)]
    D3D11StructuredBuffer,

    #[allow(missing_docs)]
    D3D11RwStructuredBuffer,

    #[allow(missing_docs)]
    D3D11AppendStructuredBuffer,

    #[allow(missing_docs)]
    D3D11ConsumeStructuredBuffer,

    /// This value is not used by a programmer; it exists to force the enumeration to compile to 32
    /// bits.
    ForceDword = 0x7fffffff,
}
