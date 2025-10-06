use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_UNORDERED_ACCESS_VIEW_DESC;

/// Describes a unordered-access 1D texture resource.
///
/// # Remarks
/// This structure is used by a [`D3D11_UNORDERED_ACCESS_VIEW_DESC`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_UAV {
    /// The mipmap slice index.
    pub mip_slice: UINT,
}

impl Default for D3D11_TEX1D_UAV {
    fn default() -> Self {
        D3D11_TEX1D_UAV { mip_slice: 0 }
    }
}
