use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_UNORDERED_ACCESS_VIEW_DESC;

/// Describes an array of unordered-access 1D texture resources.
///
/// # Remarks
/// This structure is used by a [`D3D11_UNORDERED_ACCESS_VIEW_DESC`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX1D_ARRAY_UAV {
    /// The mipmap slice index.
    pub mip_slice: UINT,

    /// The zero-based index of the first array slice to be accessed.
    pub first_array_slice: UINT,

    /// The number of slices in the array.
    pub array_slice: UINT,
}

impl Default for D3D11_TEX1D_ARRAY_UAV {
    fn default() -> Self {
        D3D11_TEX1D_ARRAY_UAV {
            mip_slice: 0,
            first_array_slice: 0,
            array_slice: 0,
        }
    }
}
