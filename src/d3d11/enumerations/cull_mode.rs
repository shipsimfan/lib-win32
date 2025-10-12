// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_RASTERIZER_DESC;

/// Indicates triangles facing a particular direction are not drawn.
///
/// # Remarks
/// This enumeration is part of a rasterizer-state object description (see
/// [`D3D11_RASTERIZER_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_CULL_MODE {
    /// Always draw all triangles.
    None = 1,

    /// Do not draw triangles that are front-facing.
    Front = 2,

    /// Do not draw triangles that are back-facing.
    Back = 3,
}
