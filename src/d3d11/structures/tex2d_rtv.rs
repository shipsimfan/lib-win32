use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_RENDER_TARGET_VIEW_DESC;

/// Specifies the subresource from a 2D texture to use in a render-target view.
///
/// # Remarks
/// This structure is one member of a render-target-view description (see
/// [`D3D11_RENDER_TARGET_VIEW_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2D_RTV {
    /// The index of the mipmap level to use mip slice.
    pub mip_slice: UINT,
}

impl Default for D3D11_TEX2D_RTV {
    fn default() -> Self {
        D3D11_TEX2D_RTV { mip_slice: 0 }
    }
}
