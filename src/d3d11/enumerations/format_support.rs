// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11DeviceContext};

/// Which resources are supported for a given format and given device (see
/// [`ID3D11Device::check_format_support`] and [`ID3D11Device::check_feature_support`]).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_FORMAT_SUPPORT {
    /// Buffer resources supported.
    Buffer = 0x1,

    /// Vertex buffers supported.
    IaVertexBuffer = 0x2,

    /// Index buffers supported.
    IaIndexBuffer = 0x4,

    /// Streaming output buffers supported.
    SoBuffer = 0x8,

    /// 1D texture resources supported.
    Texture1D = 0x10,

    /// 2D texture resources supported.
    Texture2D = 0x20,

    /// 3D texture resources supported.
    Texture3D = 0x40,

    /// Cube texture resources supported.
    Texturecube = 0x80,

    /// The HLSL Load function for texture objects is supported.
    ShaderLoad = 0x100,

    /// The HLSL Sample function for texture objects is supported.
    ShaderSample = 0x200,

    /// The HLSL SampleCmp and SampleCmpLevelZero functions for texture objects are supported.
    ShaderSampleComparison = 0x400,

    /// Reserved.
    ShaderSampleMonoText = 0x800,

    /// Mipmaps are supported.
    Mip = 0x1000,

    /// Automatic generation of mipmaps is supported.
    MipAutogen = 0x2000,

    /// Render targets are supported.
    RenderTarget = 0x4000,

    /// Blend operations supported.
    Blendable = 0x8000,

    /// Depth stencils supported.
    DepthStencil = 0x10000,

    /// CPU locking supported.
    CpuLockable = 0x20000,

    /// Multisample antialiasing (MSAA) resolve operations are supported. For more info, see
    /// [`ID3D11DeviceContext::resolve_subresource`].
    MultisampleResolve = 0x40000,

    /// Format can be displayed on screen.
    Display = 0x80000,

    /// Format cannot be cast to another format.
    CastWithinBitLayout = 0x100000,

    /// Format can be used as a multisampled rendertarget.
    MultisampleRendertarget = 0x200000,

    /// Format can be used as a multisampled texture and read into a shader with the HLSL load
    /// function.
    MultisampleLoad = 0x400000,

    /// Format can be used with the HLSL gather function. This value is available in DirectX 10.1
    /// or higher.
    ShaderGather = 0x800000,

    /// Format supports casting when the resource is a back buffer.
    BackBufferCast = 0x1000000,

    /// Format can be used for an unordered access view.
    TypedUnorderedAccessView = 0x2000000,

    /// Format can be used with the HLSL gather with comparison function.
    ShaderGatherComparison = 0x4000000,

    /// Format can be used with the decoder output.
    DecoderOutput = 0x8000000,

    /// Format can be used with the video processor output.
    VideoProcessorOutput = 0x10000000,

    /// Format can be used with the video processor input.
    VideoProcessorInput = 0x20000000,

    /// Format can be used with the video encoder.
    VideoEncoder = 0x40000000,
}
