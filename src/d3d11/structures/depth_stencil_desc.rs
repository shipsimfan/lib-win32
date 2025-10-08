use crate::{
    d3d11::{
        D3D11_COMPARISON_FUNC, D3D11_DEFAULT_STENCIL_READ_MASK, D3D11_DEFAULT_STENCIL_WRITE_MASK,
        D3D11_DEPTH_STENCILOP_DESC, D3D11_DEPTH_WRITE_MASK,
    },
    BOOL, FALSE, TRUE, UINT8,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{d3d11::ID3D11Device, dxgi::DXGI_FORMAT};

/// Describes depth-stencil state.
///
/// # Remarks
/// Pass a pointer to [`D3D11_DEPTH_STENCIL_DESC`] to the
/// [`ID3D11Device::create_depth_stencil_state`] method to create the depth-stencil state object.
///
/// Depth-stencil state controls how depth-stencil testing is performed by the output-merger stage.
///
/// The formats that support stenciling are [`DXGI_FORMAT::D24UNormS8UInt`] and
/// [`DXGI_FORMAT::D32FloatS8X24UInt`].
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct D3D11_DEPTH_STENCIL_DESC {
    /// Enable depth testing.
    pub depth_enable: BOOL,

    /// Identify a portion of the depth-stencil buffer that can be modified by depth data (see
    /// [`D3D11_DEPTH_WRITE_MASK`]).
    pub depth_write_mask: D3D11_DEPTH_WRITE_MASK,

    /// A function that compares depth data against existing depth data. The function options are
    /// listed in [`D3D11_COMPARISON_FUNC`].
    pub depth_func: D3D11_COMPARISON_FUNC,

    /// Enable stencil testing.
    pub stencil_enable: BOOL,

    /// Identify a portion of the depth-stencil buffer for reading stencil data.
    pub stencil_read_mask: UINT8,

    /// Identify a portion of the depth-stencil buffer for writing stencil data.
    pub stencil_write_mask: UINT8,

    /// Identify how to use the results of the depth test and the stencil test for pixels whose
    /// surface normal is facing towards the camera (see [`D3D11_DEPTH_STENCILOP_DESC`]).
    pub front_face: D3D11_DEPTH_STENCILOP_DESC,

    /// Identify how to use the results of the depth test and the stencil test for pixels whose
    /// surface normal is facing away from the camera (see [`D3D11_DEPTH_STENCILOP_DESC`]).
    pub back_face: D3D11_DEPTH_STENCILOP_DESC,
}

impl Default for D3D11_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        D3D11_DEPTH_STENCIL_DESC {
            depth_enable: TRUE,
            depth_write_mask: D3D11_DEPTH_WRITE_MASK::All,
            depth_func: D3D11_COMPARISON_FUNC::Less,
            stencil_enable: FALSE,
            stencil_read_mask: D3D11_DEFAULT_STENCIL_READ_MASK,
            stencil_write_mask: D3D11_DEFAULT_STENCIL_WRITE_MASK,
            front_face: D3D11_DEPTH_STENCILOP_DESC::default(),
            back_face: D3D11_DEPTH_STENCILOP_DESC::default(),
        }
    }
}
