use crate::{d3d11::D3D11_USAGE, UINT};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{
    ID3D11Device, D3D11_BIND_FLAG, D3D11_CPU_ACCESS_FLAG, D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT,
    D3D11_RESOURCE_MISC_FLAG,
};

/// Describes a buffer resource.
///
/// # Remarks
/// This structure is used by [`ID3D11Device::create_buffer`] to create buffer resources.
///
/// If the bind flag is [`D3D11_BIND_FLAG::ConstantBuffer`], you must set the `byte_width` value in
/// multiples of 16, and less than or equal to [`D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_BUFFER_DESC {
    /// Size of the buffer in bytes.
    pub byte_width: UINT,

    /// Identify how the buffer is expected to be read from and written to. Frequency of update is
    /// a key factor. The most common value is typically [`D3D11_USAGE::Default`].
    pub usage: D3D11_USAGE,

    /// Identify how the buffer will be bound to the pipeline. Flags (see [`D3D11_BIND_FLAG`]) can
    /// be combined with a bitwise OR.
    pub bind_flags: UINT,

    /// CPU access flags (see [`D3D11_CPU_ACCESS_FLAG`]) or 0 if no CPU access is necessary. Flags
    /// can be combined with a bitwise OR.
    pub cpu_access_flags: UINT,

    /// Miscellaneous flags (see [`D3D11_RESOURCE_MISC_FLAG`]) or 0 if unused. Flags can be
    /// combined with a bitwise OR.
    pub misc_flags: UINT,

    /// The size of each element in the buffer structure (in bytes) when the buffer represents a
    /// structured buffer.
    ///
    /// The size value in `structure_byte_stride` must match the size of the format that you use
    /// for views of the buffer. For example, if you use a shader resource view (SRV) to read a
    /// buffer in a pixel shader, the SRV format size must match the size value in
    /// `structure_byte_stride`.
    pub structure_byte_stride: UINT,
}

impl Default for D3D11_BUFFER_DESC {
    fn default() -> Self {
        D3D11_BUFFER_DESC {
            byte_width: 0,
            usage: D3D11_USAGE::Default,
            bind_flags: 0,
            cpu_access_flags: 0,
            misc_flags: 0,
            structure_byte_stride: 0,
        }
    }
}
