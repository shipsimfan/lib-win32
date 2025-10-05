use crate::{
    d3d11::{
        D3D11_BUFFEREX_SRV, D3D11_BUFFER_SRV, D3D11_SRV_DIMENSION, D3D11_TEX1D_ARRAY_SRV,
        D3D11_TEX1D_SRV, D3D11_TEX2DMS_ARRAY_SRV, D3D11_TEX2DMS_SRV, D3D11_TEX2D_ARRAY_SRV,
        D3D11_TEX2D_SRV, D3D11_TEX3D_SRV, D3D11_TEXCUBE_ARRAY_SRV, D3D11_TEXCUBE_SRV,
    },
    dxgi::DXGI_FORMAT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Device, ID3D11ShaderResourceView};

/// Describes a shader-resource view.
///
/// # Remarks
/// A view is a format-specific way to look at the data in a resource. The view determines what
/// data to look at, and how it is cast when read.
///
/// When viewing a resource, the resource-view description must specify a typed format, that is
/// compatible with the resource format. So that means that you cannot create a resource-view
/// description using any format with `_TYPELESS` in the name. You can however view a typeless
/// resource by specifying a typed format for the view. For example, a
/// [`DXGI_FORMAT::R32G32B32Typeless`] resource can be viewed with one of these typed formats:
/// [`DXGI_FORMAT::R32G32B32Float`], [`DXGI_FORMAT::R32G32B32UInt`], and
/// [`DXGI_FORMAT::R32G32B32SInt`], since these typed formats are compatible with the typeless
/// resource.
///
/// Create a shader-resource-view description by calling
/// [`ID3D11Device::create_shader_resource_view`]. To view a shader-resource-view description, call
/// [`ID3D11ShaderResourceView::get_desc`].
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_SHADER_RESOURCE_VIEW_DESC {
    /// A DXGI_FORMAT specifying the viewing format. See remarks.
    pub format: DXGI_FORMAT,

    /// The resource type of the view. See [`D3D11_SRV_DIMENSION`]. You must set `view_dimension`
    /// to the same resource type as that of the underlying resource. This parameter also
    /// determines which `_SRV` to use in the union below.
    pub view_dimension: D3D11_SRV_DIMENSION,

    #[allow(missing_docs)]
    pub u: D3D11_SHADER_RESOURCE_VIEW_DESC_UNION,
}

impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        D3D11_SHADER_RESOURCE_VIEW_DESC {
            format: DXGI_FORMAT::Unknown,
            view_dimension: D3D11_SRV_DIMENSION::Unknown,
            u: D3D11_SHADER_RESOURCE_VIEW_DESC_UNION::default(),
        }
    }
}

/// Union of dimensions for [`D3D11_SHADER_RESOURCE_VIEW_DESC`]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union D3D11_SHADER_RESOURCE_VIEW_DESC_UNION {
    /// View the resource as a buffer using information from a shader-resource view (see
    /// [`D3D11_BUFFER_SRV`]).
    pub buffer: D3D11_BUFFER_SRV,

    /// View the resource as a 1D texture using information from a shader-resource view (see
    /// [`D3D11_TEX1D_SRV`]).
    pub texture_1d: D3D11_TEX1D_SRV,

    /// View the resource as a 1D-texture array using information from a shader-resource view (see
    /// [`D3D11_TEX1D_ARRAY_SRV`]).
    pub texture_1d_array: D3D11_TEX1D_ARRAY_SRV,

    /// View the resource as a 2D-texture using information from a shader-resource view (see
    /// [`D3D11_TEX2D_SRV`]).
    pub texture_2d: D3D11_TEX2D_SRV,

    /// View the resource as a 2D-texture array using information from a shader-resource view (see
    /// [`D3D11_TEX2D_ARRAY_SRV`]).
    pub texture_2d_array: D3D11_TEX2D_ARRAY_SRV,

    /// View the resource as a 2D-multisampled texture using information from a shader-resource
    /// view (see [`D3D11_TEX2DMS_SRV`]).
    pub texture_2dms: D3D11_TEX2DMS_SRV,

    /// View the resource as a 2D-multisampled-texture array using information from a
    /// shader-resource view (see [`D3D11_TEX2DMS_ARRAY_SRV`]).
    pub texture_2dms_array: D3D11_TEX2DMS_ARRAY_SRV,

    /// View the resource as a 3D texture using information from a shader-resource view (see
    /// [`D3D11_TEX3D_SRV`]).
    pub texture_3d: D3D11_TEX3D_SRV,

    /// View the resource as a 3D-cube texture using information from a shader-resource view (see
    /// [`D3D11_TEXCUBE_SRV`]).
    pub texture_cube: D3D11_TEXCUBE_SRV,

    /// View the resource as a 3D-cube-texture array using information from a shader-resource view
    /// (see [`D3D11_TEXCUBE_ARRAY_SRV`]).
    pub texture_cube_array: D3D11_TEXCUBE_ARRAY_SRV,

    /// View the resource as a raw buffer using information from a shader-resource view (see
    /// [`D3D11_BUFFEREX_SRV`]).
    pub buffer_ex: D3D11_BUFFEREX_SRV,
}

impl Default for D3D11_SHADER_RESOURCE_VIEW_DESC_UNION {
    fn default() -> Self {
        D3D11_SHADER_RESOURCE_VIEW_DESC_UNION {
            buffer: D3D11_BUFFER_SRV::default(),
        }
    }
}
