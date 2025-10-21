// rustdoc imports
#[allow(unused_imports)]
#[cfg(feature = "d3d11")]
use crate::d3d11::D3D11_SHADER_RESOURCE_VIEW_DESC;
#[allow(unused_imports)]
#[cfg(feature = "d3d11shader")]
use crate::d3d11shader::D3D11_SHADER_INPUT_BIND_DESC;

/// Values that identify the type of resource to be viewed as a shader resource.
///
/// # Remarks
/// A [`D3D_SRV_DIMENSION`]-typed value is specified in the `view_dimension` member of the
/// [`D3D11_SHADER_RESOURCE_VIEW_DESC`] structure or the `dimension` member of the
/// [`D3D11_SHADER_INPUT_BIND_DESC`] structure.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D_SRV_DIMENSION {
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

    #[allow(missing_docs)]
    D3D10Unknown,

    #[allow(missing_docs)]
    D3D10Buffer,

    #[allow(missing_docs)]
    D3D10Texture1D,

    #[allow(missing_docs)]
    D3D10Texture1DArray,

    #[allow(missing_docs)]
    D3D10Texture2D,

    #[allow(missing_docs)]
    D3D10Texture2DArray,

    #[allow(missing_docs)]
    D3D10Texture2DMS,

    #[allow(missing_docs)]
    D3D10Texture2DMSArray,

    #[allow(missing_docs)]
    D3D10Texture3D,

    #[allow(missing_docs)]
    D3D10TextureCube,

    #[allow(missing_docs)]
    D3D10_1Unknown,

    #[allow(missing_docs)]
    D3D10_1Buffer,

    #[allow(missing_docs)]
    D3D10_1Texture1D,

    #[allow(missing_docs)]
    D3D10_1Texture1DArray,

    #[allow(missing_docs)]
    D3D10_1Texture2D,

    #[allow(missing_docs)]
    D3D10_1Texture2DArray,

    #[allow(missing_docs)]
    D3D10_1Texture2DMS,

    #[allow(missing_docs)]
    D3D10_1Texture2DMSArray,

    #[allow(missing_docs)]
    D3D10_1Texture3D,

    #[allow(missing_docs)]
    D3D10_1TextureCube,

    #[allow(missing_docs)]
    D3D10_1TextureCubeArray,

    #[allow(missing_docs)]
    D3D11Unknown,

    #[allow(missing_docs)]
    D3D11Buffer,

    #[allow(missing_docs)]
    D3D11Texture1D,

    #[allow(missing_docs)]
    D3D11Texture1DArray,

    #[allow(missing_docs)]
    D3D11Texture2D,

    #[allow(missing_docs)]
    D3D11Texture2DArray,

    #[allow(missing_docs)]
    D3D11Texture2DMS,

    #[allow(missing_docs)]
    D3D11Texture2DMSArray,

    #[allow(missing_docs)]
    D3D11Texture3D,

    #[allow(missing_docs)]
    D3D11TextureCube,

    #[allow(missing_docs)]
    D3D11TextureCubeArray,

    #[allow(missing_docs)]
    D3D11BufferEx,
}
