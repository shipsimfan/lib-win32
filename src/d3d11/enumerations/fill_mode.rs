// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_RASTERIZER_DESC;

/// Determines the fill mode to use when rendering triangles.
///
/// # Remarks
/// This enumeration is part of a rasterizer-state object description (see
/// [`D3D11_RASTERIZER_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_FILL_MODE {
    /// Draw lines connecting the vertices. Adjacent vertices are not drawn.
    Wireframe = 2,

    /// Fill the triangles formed by the vertices. Adjacent vertices are not drawn.
    Solid = 3,
}
