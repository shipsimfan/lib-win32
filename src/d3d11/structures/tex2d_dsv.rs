use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_DEPTH_STENCIL_VIEW_DESC;

/// Specifies the subresource from a 2D texture that is accessible to a depth-stencil view.
///
/// # Remarks
/// This structure is one member of a depth-stencil-view description (see
/// [`D3D11_DEPTH_STENCIL_VIEW_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_DSV {
    /// The index of the first mipmap level to use.
    pub mip_slice: UINT,
}

impl Default for D3D11_TEX2D_DSV {
    fn default() -> Self {
        D3D11_TEX2D_DSV { mip_slice: 0 }
    }
}
