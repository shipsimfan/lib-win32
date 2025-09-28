use crate::LONG;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::IDXGIResource;

/// The surface or resource is used as a back buffer. You don’t need to pass
/// [`DXGI_USAGE_BACK_BUFFER`] when you create a swap chain. But you can determine whether a
/// resource belongs to a swap chain when you call [`IDXGIResource::get_usage`] and get
/// [`DXGI_USAGE_BACK_BUFFER`].
pub const DXGI_USAGE_BACK_BUFFER: LONG = 1 << (2 + 4);

/// This flag is for internal use only.
pub const DXGI_USAGE_DISCARD_ON_PRESENT: LONG = 1 << (5 + 4);

/// Use the surface or resource for reading only.
pub const DXGI_USAGE_READ_ONLY: LONG = 1 << (4 + 4);

/// Use the surface or resource as an output render target.
pub const DXGI_USAGE_RENDER_TARGET_OUTPUT: LONG = 1 << (1 + 4);

/// Use the surface or resource as an input to a shader.
pub const DXGI_USAGE_SHADER_INPUT: LONG = 1 << (0 + 4);

/// Share the surface or resource.
pub const DXGI_USAGE_SHARED: LONG = 1 << (3 + 4);

/// Use the surface or resource for unordered access.
pub const DXGI_USAGE_UNORDERED_ACCESS: LONG = 1 << (6 + 4);
