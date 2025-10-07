use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_RENDER_TARGET_VIEW_DESC;

/// Specifies the subresources from an array of 1D textures to use in a render-target view.
///
/// # Remarks
/// This structure is one member of a render-target-view description (see
/// [`D3D11_RENDER_TARGET_VIEW_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_ARRAY_RTV {
    /// The index of the mipmap level to use mip slice.
    pub mip_slice: UINT,

    /// The index of the first texture to use in an array of textures.
    pub first_array_slice: UINT,

    /// Number of textures to use.
    pub array_size: UINT,
}

impl Default for D3D11_TEX1D_ARRAY_RTV {
    fn default() -> Self {
        D3D11_TEX1D_ARRAY_RTV {
            mip_slice: 0,
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
