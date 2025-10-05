// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{
    D3D11_BUFFER_DESC, D3D11_RESOURCE_MISC_FLAG, D3D11_TEXTURE1D_DESC, D3D11_TEXTURE2D_DESC,
    D3D11_USAGE,
};

/// Specifies the types of CPU access allowed for a resource.
///
/// # Remarks
/// This enumeration is used in [`D3D11_BUFFER_DESC`], [`D3D11_TEXTURE1D_DESC`],
/// [`D3D11_TEXTURE2D_DESC`], [`D3D11_TEXTURE3D_DESC`].
///
/// Applications may combine one or more of these flags with a bitwise OR. When possible, create
/// resources with no CPU access flags, as this enables better resource optimization.
///
/// The [`D3D11_RESOURCE_MISC_FLAG`] cannot be used when creating resources with
/// [`D3D11_CPU_ACCESS_FLAG`]s.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_CPU_ACCESS_FLAG {
    /// The resource is to be mappable so that the CPU can change its contents. Resources created
    /// with this flag cannot be set as outputs of the pipeline and must be created with either
    /// dynamic or staging usage (see [`D3D11_USAGE`]).
    Write = 0x10000,

    /// The resource is to be mappable so that the CPU can read its contents. Resources created
    /// with this flag cannot be set as either inputs or outputs to the pipeline and must be
    /// created with staging usage (see [`D3D11_USAGE`]).
    Read = 0x20000,
}
