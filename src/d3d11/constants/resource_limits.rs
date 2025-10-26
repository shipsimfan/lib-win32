use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::{ID3D11Texture1D, ID3D11Texture2D, ID3D11Texture3D};

/// The maximum number of constant buffer slots
pub const D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT: UINT = 14;

/// The maximum number of shader resources that can be attached to one shader
pub const D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT: UINT = 128;

/// The maximum number of samplers that can be attach to one shader
pub const D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT: UINT = 16;

/// The maximum thread groups a single dimension may have for compute shaders
pub const D3D11_CS_DISPATCH_MAX_THREAD_GROUPS_PER_DIMENSION: UINT = 65535;

/// The maximum number of vertex buffers that can be attached at one time
pub const D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT: UINT = 32;

/// The maximum number of render targets that can be simultaneously bound
pub const D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT: UINT = 8;

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

/// Maximum number of SO buffer slots
pub const D3D11_SO_BUFFER_SLOT_COUNT: UINT = 4;

/// Indicates and SO stream which is not rasterized
pub const D3D11_SO_NO_RASTERIZED_STREAM: UINT = 0xFFFFFFFF;

/// Maximum number of SO output components
pub const D3D11_SO_OUTPUT_COMPONENT_COUNT: UINT = 128;

/// Maximum number of SO streams
pub const D3D11_SO_STREAM_COUNT: UINT = 4;
