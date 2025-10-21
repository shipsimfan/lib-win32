// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11shader")]
use crate::d3d11shader::D3D11_SHADER_INPUT_BIND_DESC;

/// Values that identify resource types that can be bound to a shader and that are reflected as
/// part of the resource description for the shader.
///
/// # Remarks
/// [`D3D_SHADER_INPUT_TYPE`]-typed values are specified in the Type member of the
/// [`D3D11_SHADER_INPUT_BIND_DESC`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D_SHADER_INPUT_TYPE {
    /// The shader resource is a constant buffer.
    CBuffer = 0,

    /// The shader resource is a texture buffer.
    TBuffer,

    /// The shader resource is a texture.
    Texture,

    /// The shader resource is a sampler.
    Sampler,

    /// The shader resource is a read-and-write buffer or texture.
    UavRwTyped,

    /// The shader resource is a structured buffer.
    Structured,

    /// The shader resource is a read-and-write structured buffer.
    UavRwStructured,

    /// The shader resource is a byte-address buffer.
    ByteAddress,

    /// The shader resource is a read-and-write byte-address buffer.
    UavRwByteAddress,

    /// The shader resource is an append-structured buffer.
    UavAppendStructured,

    /// The shader resource is a consume-structured buffer.
    UavConsumeStructured,

    /// The shader resource is a read-and-write structured buffer that uses the built-in counter to
    /// Sappend or consume.
    UavRwStructuredWithCounter,

    #[allow(missing_docs)]
    RtcAccelerationsStructure,

    #[allow(missing_docs)]
    UavFeedBackTexture,

    /// The shader resource is a constant buffer.
    D3D10CBuffer,

    /// The shader resource is a texture buffer.
    D3D10TBuffer,

    /// The shader resource is a texture.
    D3D10Texture,

    /// The shader resource is a sampler.
    D3D10Sampler,

    /// The shader resource is a read-and-write buffer.
    D3D11UavRwTyped,

    /// The shader resource is a structured buffer.
    D3D11Structured,

    /// The shader resource is a read-and-write structured buffer.
    D3D11UavRwStructured,

    /// The shader resource is a byte-address buffer.
    D3D11ByteAddress,

    /// The shader resource is a read-and-write byte-address buffer.
    D3D11UavRwByteAddress,

    /// The shader resource is an append-structured buffer.
    D3D11UavAppendStructured,

    /// The shader resource is a consume-structured buffer.
    D3D11UavConsumeStructured,

    /// The shader resource is a read-and-write structured buffer that uses the built-in counter to
    /// append or consume.
    D3D11UavRwStructuredWithCounter,
}
