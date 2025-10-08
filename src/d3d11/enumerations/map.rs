// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11DeviceContext, D3D11_BIND_FLAG, D3D11_CPU_ACCESS_FLAG, D3D11_USAGE};

/// Identifies a resource to be accessed for reading and writing by the CPU. Applications may
/// combine one or more of these flags.
///
/// # Remarks
/// This enumeration is used in [`ID3D11DeviceContext::map`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_MAP {
    /// Resource is mapped for reading. The resource must have been created with read access (see
    /// [`D3D11_CPU_ACCESS_FLAG::Read`]).
    Read = 1,

    /// Resource is mapped for writing. The resource must have been created with write access (see
    /// [`D3D11_CPU_ACCESS_FLAG::Write`]).
    Write = 2,

    /// Resource is mapped for reading and writing. The resource must have been created with read
    /// and write access (see [`D3D11_CPU_ACCESS_FLAG::Read`] and
    /// [`D3D11_CPU_ACCESS_FLAG::Write`]).
    ReadWrite = 3,

    /// Resource is mapped for writing; the previous contents of the resource will be undefined.
    /// The resource must have been created with write access and dynamic usage (See
    /// [`D3D11_CPU_ACCESS_FLAG::Write`] and [`D3D11_USAGE::Dynamic`]).
    WriteDiscard = 4,

    /// Resource is mapped for writing; the existing contents of the resource cannot be
    /// overwritten. This flag is only valid on vertex and index buffers. The resource must have
    /// been created with write access (see [`D3D11_CPU_ACCESS_FLAG::Write`]). Cannot be used on a
    /// resource created with the [`D3D11_BIND_FLAG::ConstantBuffer`] flag.
    WriteNoOverwrite = 5,
}
