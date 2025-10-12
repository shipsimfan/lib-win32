// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{
    D3D11_BUFFER_DESC, D3D11_TEXTURE1D_DESC, D3D11_TEXTURE2D_DESC, D3D11_TEXTURE3D_DESC,
};

/// Identifies expected resource use during rendering. The usage directly reflects whether a
/// resource is accessible by the CPU and/or the graphics processing unit (GPU).
///
/// # Remarks
/// An application identifies the way a resource is intended to be used (its usage) in a resource
/// description. There are several structures for creating resources including:
/// [`D3D11_TEXTURE1D_DESC`], [`D3D11_TEXTURE2D_DESC`], [`D3D11_TEXTURE3D_DESC`], and
/// [`D3D11_BUFFER_DESC`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_USAGE {
    /// A resource that requires read and write access by the GPU. This is likely to be the most
    /// common usage choice.
    Default = 0,

    /// A resource that can only be read by the GPU. It cannot be written by the GPU, and cannot be
    /// accessed at all by the CPU. This type of resource must be initialized when it is created,
    /// since it cannot be changed after creation.
    Immutable = 1,

    /// A resource that is accessible by both the GPU (read only) and the CPU (write only). A
    /// dynamic resource is a good choice for a resource that will be updated by the CPU at least
    /// once per frame. To update a dynamic resource, use a `map` method.
    Dynamic = 2,

    /// A resource that supports data transfer (copy) from the GPU to the CPU.
    Staging = 3,
}
