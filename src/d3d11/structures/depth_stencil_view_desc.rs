use crate::{
    d3d11::{
        D3D11_DSV_DIMENSION, D3D11_TEX1D_ARRAY_DSV, D3D11_TEX1D_DSV, D3D11_TEX2DMS_ARRAY_DSV,
        D3D11_TEX2DMS_DSV, D3D11_TEX2D_ARRAY_DSV, D3D11_TEX2D_DSV,
    },
    dxgi::DXGI_FORMAT,
    UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, D3D11_DSV_FLAG};

/// Specifies the subresources of a texture that are accessible from a depth-stencil view.
///
/// # Remarks
/// These are valid formats for a depth-stencil view:
///  - [`DXGI_FORMAT::D16UNorm`]
///  - [`DXGI_FORMAT::D24UNormS8UInt`]
///  - [`DXGI_FORMAT::D32Float`]
///  - [`DXGI_FORMAT::D32FloatS8X24UInt`]
///  - [`DXGI_FORMAT::Unknown`]
///
/// A depth-stencil view cannot use a typeless format. If the format chosen is
/// [`DXGI_FORMAT::Unknown`], then the format of the parent resource is used.
///
/// A depth-stencil-view description is needed when calling
/// [`ID3D11Device::create_depth_stencil_view`].
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_DEPTH_STENCIL_VIEW_DESC {
    /// Resource data format (see [`DXGI_FORMAT`]). See remarks for allowable formats.
    pub format: DXGI_FORMAT,

    /// Type of resource (see [`D3D11_DSV_DIMENSION`]). Specifies how a depth-stencil resource will
    /// be accessed; the value is stored in the union in this structure.
    pub view_dimension: D3D11_DSV_DIMENSION,

    /// A value that describes whether the texture is read only. Pass 0 to specify that it is not
    /// read only; otherwise, pass one of the members of the [`D3D11_DSV_FLAG`] enumerated type.
    pub flags: UINT,

    #[allow(missing_docs)]
    pub u: D3D11_DEPTH_STENCIL_VIEW_DESC_UNION,
}

impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        D3D11_DEPTH_STENCIL_VIEW_DESC {
            format: DXGI_FORMAT::Unknown,
            view_dimension: D3D11_DSV_DIMENSION::Unknown,
            flags: 0,
            u: D3D11_DEPTH_STENCIL_VIEW_DESC_UNION::default(),
        }
    }
}

/// Union of dimensions for [`D3D11_DEPTH_STENCIL_VIEW_DESC`]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union D3D11_DEPTH_STENCIL_VIEW_DESC_UNION {
    /// Specifies a 1D texture subresource (see [`D3D11_TEX1D_DSV`]).
    pub texture_1d: D3D11_TEX1D_DSV,

    /// Specifies an array of 1D texture subresources (see [`D3D11_TEX1D_ARRAY_DSV`]).
    pub texture_1d_array: D3D11_TEX1D_ARRAY_DSV,

    /// Specifies a 2D texture subresource (see [`D3D11_TEX2D_DSV`]).
    pub texture_2d: D3D11_TEX2D_DSV,

    /// Specifies an array of 2D texture subresources (see [`D3D11_TEX2D_ARRAY_DSV`]).
    pub texture_2d_array: D3D11_TEX2D_ARRAY_DSV,

    /// Specifies a multisampled 2D texture (see [`D3D11_TEX2DMS_DSV`]).
    pub texture_2dms: D3D11_TEX2DMS_DSV,

    /// Specifies an array of multisampled 2D textures (see [`D3D11_TEX2DMS_ARRAY_DSV`]).
    pub texture_2dms_array: D3D11_TEX2DMS_ARRAY_DSV,
}

impl Default for D3D11_DEPTH_STENCIL_VIEW_DESC_UNION {
    fn default() -> Self {
        D3D11_DEPTH_STENCIL_VIEW_DESC_UNION {
            texture_1d: D3D11_TEX1D_DSV::default(),
        }
    }
}
