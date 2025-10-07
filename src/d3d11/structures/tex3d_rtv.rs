use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_RENDER_TARGET_VIEW_DESC;

/// Specifies the subresources from a 3D texture to use in a render-target view.
///
/// # Remarks
/// This structure is one member of a render target view. See [`D3D11_RENDER_TARGET_VIEW_DESC`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX3D_RTV {
    /// The index of the mipmap level to use mip slice.
    pub mip_slice: UINT,

    /// First depth level to use.
    pub first_w_slice: UINT,

    /// Number of depth levels to use in the render-target view, starting from `first_w_slice`. A
    /// value of -1 indicates all of the slices along the w axis, starting from `first_w_slice`.
    pub w_size: UINT,
}

impl Default for D3D11_TEX3D_RTV {
    fn default() -> Self {
        D3D11_TEX3D_RTV {
            mip_slice: 0,
            first_w_slice: 0,
            w_size: 0,
        }
    }
}
