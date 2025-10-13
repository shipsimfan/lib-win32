// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_DEPTH_STENCIL_VIEW_DESC;

/// Depth-stencil view options.
///
/// # Remarks
/// This enumeration is used by [`D3D11_DEPTH_STENCIL_VIEW_DESC`].
///
/// Limiting a depth-stencil buffer to read-only access allows more than one depth-stencil view to
/// be bound to the pipeline simultaneously, since it is not possible to have a read/write
/// conflicts between separate views.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_DSV_FLAG {
    /// Indicates that depth values are read only.
    ReadOnlyDepth = 0x1,

    /// Indicates that stencil values are read only.
    ReadOnlyStencil = 0x2,
}
