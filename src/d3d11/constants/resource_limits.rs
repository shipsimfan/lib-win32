use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Texture1D, ID3D11Texture2D, ID3D11Texture3D};

/// Limit of the number of elements in a constant buffer
pub const D3D11_REQ_CONSTANT_BUFFER_ELEMENT_COUNT: UINT = 4096;

/// Maximum number of textures in a [`ID3D11Texture1D`]
pub const D3D11_REQ_TEXTURE1D_ARRAY_AXIS_DIMENSION: UINT = 2048;

/// Maximum width of an [`ID3D11Texture1D`]
pub const D3D11_REQ_TEXTURE1D_U_DIMENSION: UINT = 16384;

/// Maximum number of textures in a [`ID3D11Texture2D`]
pub const D3D11_REQ_TEXTURE2D_ARRAY_AXIS_DIMENSION: UINT = 2048;

/// Maximum width or height of an [`ID3D11Texture2D`]
pub const D3D11_REQ_TEXTURE2D_U_OR_V_DIMENSION: UINT = 16384;

/// Maximum width, height, or depth of an [`ID3D11Texture3D`]
pub const D3D11_REQ_TEXTURE3D_U_V_OR_W_DIMENSION: UINT = 2048;

/// Maximum width or height of a texture cube
pub const D3D11_REQ_TEXTURECUBE_DIMENSION: UINT = 16384;
