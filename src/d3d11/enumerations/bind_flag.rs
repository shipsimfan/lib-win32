// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

/// Identifies how to bind a resource to the pipeline.
///
/// # Remarks
/// In general, binding flags can be combined using a bitwise OR (except the constant-buffer flag);
/// however, you should use a single flag to allow the device to optimize the resource usage.
///
/// This enumeration is used by a:
///  - Buffer description when creating a buffer.
///  - Texture description when creating a texture (see [`D3D11_TEXTURE1D_DESC`] or
///    [`D3D11_TEXTURE2D_DESC`] or [`D3D11_TEXTURE3D_DESC`]).
///
/// A shader-resource buffer is NOT a constant buffer; rather, it is a texture or buffer resource
/// that is bound to a shader, that contains texture or buffer data (it is not limited to a single
/// element type in the buffer). A shader-resource buffer is created with the
/// [`D3D11_BIND_FLAG::ShaderResource`] flag and is bound to the pipeline using one of these APIs:
/// [`ID3D11DeviceContext::gs_set_shader_resources`],
/// [`ID3D11DeviceContext::ps_set_shader_resources`], or
/// [`ID3D11DeviceContext::vs_set_shader_resources`]. Furthermore, a shader-resource buffer cannot
/// use the [`D3D11_MAP_FLAG::WriteNoOverwrite`] flag.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_BIND_FLAG {
    /// Bind a buffer as a vertex buffer to the input-assembler stage.
    VertexBuffer = 0x1,

    /// Bind a buffer as an index buffer to the input-assembler stage.
    IndexBuffer = 0x2,

    /// Bind a buffer as a constant buffer to a shader stage; this flag may NOT be combined with
    /// any other bind flag.
    ConstantBuffer = 0x4,

    /// Bind a buffer or texture to a shader stage; this flag cannot be used with the
    /// [`D3D11_MAP_FLAG::WriteNoOverwrite`] flag.
    ShaderResource = 0x8,

    /// Bind an output buffer for the stream-output stage.
    StreamOutput = 0x10,

    /// Bind a texture as a render target for the output-merger stage.
    RenderTarget = 0x20,

    /// Bind a texture as a depth-stencil target for the output-merger stage.
    DepthStencil = 0x40,

    /// Bind an unordered access resource.
    UnorderedAccess = 0x80,

    /// Set this flag to indicate that a 2D texture is used to receive output from the decoder API.
    /// The common way to create resources for a decoder output is by calling the
    /// [`ID3D11Device::create_texture_2d`] method to create an array of 2D textures. However, you
    /// cannot use texture arrays that are created with this flag in calls to
    /// [`ID3D11Device::create_shader_resource_view`].
    Decoder = 0x200,

    /// Set this flag to indicate that a 2D texture is used to receive input from the video encoder
    /// API. The common way to create resources for a video encoder is by calling the
    /// [`ID3D11Device::create_texture_2d`] method to create an array of 2D textures. However, you
    /// cannot use texture arrays that are created with this flag in calls to
    /// [`ID3D11Device::create_shader_resource_view`].
    VideoEncoder = 0x400,
}
