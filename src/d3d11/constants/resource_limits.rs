use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11Texture1D;

/// Limit of the number of elements in a constant buffer
pub const D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: UINT = 4096;

/// Maximum number of textures in a [`ID3D11Texture1D`]
pub const D3D11_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: UINT = 2048;

/// Maximum width of an [`ID3D11Texture1D`]
pub const D3D11_REQ_TEXTURE1D_U_DIMENSION: UINT = 16384;
