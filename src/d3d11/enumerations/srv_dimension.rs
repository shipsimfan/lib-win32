// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_SHADER_RESOURCE_VIEW_DESC;

/// These flags identify the type of resource that will be viewed as a shader resource.
///
/// # Remarks
/// These flags are used by a shader-resource-view description (see
/// [`D3D11_SHADER_RESOURCE_VIEW_DESC`]).
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_SRV_DIMENSION {
    /// The type is unknown.
    Unknown = 0,

    /// The resource is a buffer.
    Buffer = 1,

    /// The resource is a 1D texture.
    Texture1D = 2,

    /// The resource is an array of 1D textures.
    Texture1DArray = 3,

    /// The resource is a 2D texture.
    Texture2D = 4,

    /// The resource is an array of 2D textures.
    Texture2DArray = 5,

    /// The resource is a multisampling 2D texture.
    Texture2DMS = 6,

    /// The resource is an array of multisampling 2D textures.
    Texture2DMSArray = 7,

    /// The resource is a 3D texture.
    Texture3D = 8,

    /// The resource is a cube texture.
    TextureCube = 9,

    /// The resource is an array of cube textures.
    TextureCubeArray = 10,

    /// The resource is a raw buffer.
    BufferEx = 11,
}
