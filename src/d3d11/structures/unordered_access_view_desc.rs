use crate::{
    d3d11::{
        D3D11_BUFFER_UAV, D3D11_TEX1D_ARRAY_UAV, D3D11_TEX1D_UAV, D3D11_TEX2D_ARRAY_UAV,
        D3D11_TEX2D_UAV, D3D11_TEX3D_UAV, D3D11_UAV_DIMENSION,
    },
    dxgi::DXGI_FORMAT,
};
use std::ops::{Deref, DerefMut};

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Device;

/// Specifies the subresources from a resource that are accessible using an unordered-access view.
///
/// # Remarks
/// An unordered-access-view description is passed into
/// [`ID3D11Device::create_unordered_access_view`] to create a view.
#[repr(C)]
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_UNORDERED_ACCESS_VIEW_DESC {
    /// The data format (see [`DXGI_FORMAT`]).
    pub format: DXGI_FORMAT,

    /// The resource type (see [`D3D11_UAV_DIMENSION`]), which specifies how the resource will be
    /// accessed.
    pub view_dimension: D3D11_UAV_DIMENSION,

    #[allow(missing_docs)]
    pub u: D3D11_UNORDERED_ACCESS_VIEW_DESC_UNION,
}

impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        D3D11_UNORDERED_ACCESS_VIEW_DESC {
            format: DXGI_FORMAT::Unknown,
            view_dimension: D3D11_UAV_DIMENSION::Unknown,
            u: D3D11_UNORDERED_ACCESS_VIEW_DESC_UNION::default(),
        }
    }
}

impl Deref for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    type Target = D3D11_UNORDERED_ACCESS_VIEW_DESC_UNION;

    fn deref(&self) -> &Self::Target {
        &self.u
    }
}

impl DerefMut for D3D11_UNORDERED_ACCESS_VIEW_DESC {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.u
    }
}

/// Union of dimensions for [`D3D11_UNORDERED_ACCESS_VIEW_DESC`]
#[repr(C)]
#[derive(Clone, Copy)]
#[allow(non_camel_case_types)]
pub union D3D11_UNORDERED_ACCESS_VIEW_DESC_UNION {
    /// Specifies which buffer elements can be accessed (see [`D3D11_BUFFER_UAV`]).
    pub buffer: D3D11_BUFFER_UAV,

    /// Specifies the subresources in a 1D texture that can be accessed (see [`D3D11_TEX1D_UAV`]).
    pub texture_1d: D3D11_TEX1D_UAV,

    /// Specifies the subresources in a 1D texture array that can be accessed (see
    /// [`D3D11_TEX1D_ARRAY_UAV`]).
    pub texture_1d_array: D3D11_TEX1D_ARRAY_UAV,

    /// Specifies the subresources in a 2D texture that can be accessed (see [`D3D11_TEX2D_UAV`]).
    pub texture_2d: D3D11_TEX2D_UAV,

    /// Specifies the subresources in a 2D texture array that can be accessed (see
    /// [`D3D11_TEX2D_ARRAY_UAV`]).
    pub texture_2d_array: D3D11_TEX2D_ARRAY_UAV,

    /// Specifies subresources in a 3D texture that can be accessed (see [`D3D11_TEX3D_UAV`]).
    pub texture_3d: D3D11_TEX3D_UAV,
}

impl Default for D3D11_UNORDERED_ACCESS_VIEW_DESC_UNION {
    fn default() -> Self {
        D3D11_UNORDERED_ACCESS_VIEW_DESC_UNION {
            buffer: D3D11_BUFFER_UAV::default(),
        }
    }
}
