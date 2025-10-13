use crate::RECT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11DeviceContext;

/// A [`RECT`]
///
/// # Remarks
/// This structure is used for scissor rectangles by [`ID3D11DeviceContext::rs_get_scissor_rects`]
/// and [`ID3D11DeviceContext::rs_set_scissor_rects`].
#[allow(non_camel_case_types)]
pub type D3D11_RECT = RECT;
