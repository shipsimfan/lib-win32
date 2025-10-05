use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_SHADER_RESOURCE_VIEW_DESC;

/// Specifies the subresources from an array of multisampled 2D textures to use in a
/// shader-resource view.
///
/// # Remarks
/// This structure is one member of a shader-resource-view description (see
/// [`D3D11_SHADER_RESOURCE_VIEW_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct D3D11_TEX2DMS_ARRAY_SRV {
    /// The index of the first texture to use in an array of textures.
    pub first_array_slice: UINT,

    /// Number of textures to use.
    pub array_size: UINT,
}

impl Default for D3D11_TEX2DMS_ARRAY_SRV {
    fn default() -> Self {
        D3D11_TEX2DMS_ARRAY_SRV {
            first_array_slice: 0,
            array_size: 0,
        }
    }
}
