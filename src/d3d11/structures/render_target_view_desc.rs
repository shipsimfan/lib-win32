use crate::{
    d3d11::{
        D3D11_BUFFER_RTV, D3D11_RTV_DIMENSION, D3D11_TEX1D_ARRAY_RTV, D3D11_TEX1D_RTV,
        D3D11_TEX2DMS_ARRAY_RTV, D3D11_TEX2DMS_RTV, D3D11_TEX2D_ARRAY_RTV, D3D11_TEX2D_RTV,
        D3D11_TEX3D_RTV,
    },
    dxgi::DXGI_FORMAT,
};
use std::ops::{Deref, DerefMut};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

/// Specifies the subresources from a resource that are accessible using a render-target view.
///
/// # Remarks
/// A render-target-view description is passed into [`ID3D11Device::create_render_target_view`] to
/// create a render target.
///
/// A render-target-view cannot use the following formats:
///  - Any typeless format.
///  - `DXGI_FORMAT::R32G32B32*` if the view will be used to bind a buffer (vertex, index,
///    constant, or stream-output).
///
/// If the format is set to [`DXGI_FORMAT::Unknown`], then the format of the resource that the view
/// binds to the pipeline will be used.
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_RENDER_TARGET_VIEW_DESC {
    /// The data format (see [`DXGI_FORMAT`]).
    pub format: DXGI_FORMAT,

    /// The resource type (see [`D3D11_RTV_DIMENSION`]), which specifies how the render-target
    /// resource will be accessed.
    pub view_dimension: D3D11_RTV_DIMENSION,

    #[allow(missing_docs)]
    pub u: D3D11_RENDER_TARGET_VIEW_DESC_UNION,
}

impl Default for D3D11_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        D3D11_RENDER_TARGET_VIEW_DESC {
            format: DXGI_FORMAT::Unknown,
            view_dimension: D3D11_RTV_DIMENSION::Unknown,
            u: D3D11_RENDER_TARGET_VIEW_DESC_UNION::default(),
        }
    }
}

impl Deref for D3D11_RENDER_TARGET_VIEW_DESC {
    type Target = D3D11_RENDER_TARGET_VIEW_DESC_UNION;

    fn deref(&self) -> &Self::Target {
        &self.u
    }
}

impl DerefMut for D3D11_RENDER_TARGET_VIEW_DESC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.u
    }
}

/// Union of dimensions for [`D3D11_RENDER_TARGET_VIEW_DESC`]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union D3D11_RENDER_TARGET_VIEW_DESC_UNION {
    /// Specifies which buffer elements can be accessed (see [`D3D11_BUFFER_RTV`]).
    pub buffer: D3D11_BUFFER_RTV,

    /// Specifies the subresources in a 1D texture that can be accessed (see [`D3D11_TEX1D_RTV`]).
    pub texture_1d: D3D11_TEX1D_RTV,

    /// Specifies the subresources in a 1D texture array that can be accessed (see
    /// [`D3D11_TEX1D_ARRAY_RTV`]).
    pub texture_1d_array: D3D11_TEX1D_ARRAY_RTV,

    /// Specifies the subresources in a 2D texture that can be accessed (see [`D3D11_TEX2D_RTV`]).
    pub texture_2d: D3D11_TEX2D_RTV,

    /// Specifies the subresources in a 2D texture array that can be accessed (see
    /// [`D3D11_TEX2D_ARRAY_RTV`]).
    pub texture_2d_array: D3D11_TEX2D_ARRAY_RTV,

    /// Specifies a single subresource because a multisampled 2D texture only contains one
    /// subresource (see [`D3D11_TEX2DMS_RTV`]).
    pub texture_2dms: D3D11_TEX2DMS_RTV,

    /// Specifies the subresources in a multisampled 2D texture array that can be accessed (see
    /// [`D3D11_TEX2DMS_ARRAY_RTV`]).
    pub texture_2dms_array: D3D11_TEX2DMS_ARRAY_RTV,

    /// Specifies subresources in a 3D texture that can be accessed (see [`D3D11_TEX3D_RTV`]).
    pub texture_3d: D3D11_TEX3D_RTV,
}

impl Default for D3D11_RENDER_TARGET_VIEW_DESC_UNION {
    fn default() -> Self {
        D3D11_RENDER_TARGET_VIEW_DESC_UNION {
            buffer: D3D11_BUFFER_RTV::default(),
        }
    }
}
