// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_DEPTH_STENCIL_VIEW_DESC;

/// Specifies how to access a resource used in a depth-stencil view.
///
/// # Remarks
/// This enumeration is used in [`D3D11_DEPTH_STENCIL_VIEW_DESC`] to create a depth-stencil view.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_DSV_DIMENSION {
    /// [`D3D11_DSV_DIMENSION::Unknown`] is not a valid value for [`D3D11_DEPTH_STENCIL_VIEW_DESC`]
    /// and is not used.
    Unknown = 0,

    /// The resource will be accessed as a 1D texture.
    Texture1D = 1,

    /// The resource will be accessed as an array of 1D textures.
    Texture1DArray = 2,

    /// The resource will be accessed as a 2D texture.
    Texture2D = 3,

    /// The resource will be accessed as an array of 2D textures.
    Texture2DArray = 4,

    /// The resource will be accessed as a 2D texture with multisampling.
    Texture2DMS = 5,

    /// The resource will be accessed as an array of 2D textures with multisampling.
    Texture2DMSArray = 6,
}
