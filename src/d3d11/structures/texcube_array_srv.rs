use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, D3D11_SHADER_RESOURCE_VIEW_DESC, D3D11_TEX1D_SRV};

/// Specifies the subresources from an array of cube textures to use in a shader-resource view.
///
/// # Remarks
/// This structure is one member of a shader-resource-view description (see
/// [`D3D11_SHADER_RESOURCE_VIEW_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEXCUBE_ARRAY_SRV {
    /// Index of the most detailed mipmap level to use; this number is between 0 and
    /// `mip_levels - 1` (from the original Texture Cube for which
    /// [`ID3D11Device::create_shader_resource_view`] creates a view).
    pub most_detailed_mip: UINT,

    /// The maximum number of mipmap levels for the view of the texture. See the remarks in
    /// [`D3D11_TEX1D_SRV`].
    ///
    /// Set to -1 to indicate all the mipmap levels from `most_detailed_mip` on down to least
    /// detailed.
    pub mip_levels: UINT,

    /// Index of the first 2D texture to use.
    pub first_2d_array_face: UINT,

    /// Number of cube textures in the array.
    pub num_cubes: UINT,
}

impl Default for D3D11_TEXCUBE_ARRAY_SRV {
    fn default() -> Self {
        D3D11_TEXCUBE_ARRAY_SRV {
            most_detailed_mip: 0,
            mip_levels: 0,
            first_2d_array_face: 0,
            num_cubes: 0,
        }
    }
}
