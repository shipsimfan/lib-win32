// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_RENDER_TARGET_VIEW_DESC;

/// These flags identify the type of resource that will be viewed as a render target.
///
/// # Remarks
/// This enumeration is used in [`D3D11_RENDER_TARGET_VIEW_DESC`] to create a render-target view.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_RTV_DIMENSION {
    /// Do not use this value, as it will cause [`ID3D11Device::create_render_target_view`] to
    /// fail.
    Unknown = 0,

    /// The resource will be accessed as a buffer.
    Buffer = 1,

    /// The resource will be accessed as a 1D texture.
    Texture1D = 2,

    /// The resource will be accessed as an array of 1D textures.
    Texture1DArray = 3,

    /// The resource will be accessed as a 2D texture.
    Texture2D = 4,

    /// The resource will be accessed as an array of 2D textures.
    Texture2DArray = 5,

    /// The resource will be accessed as a 2D texture with multisampling.
    Texture2DMS = 6,

    /// The resource will be accessed as an array of 2D textures with multisampling.
    Texture2DMSArray = 7,

    /// The resource will be accessed as a 3D texture.
    Texture3D = 8,
}
