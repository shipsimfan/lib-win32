// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::ID3D11DeviceContext;

/// Specifies the parts of the depth stencil to clear.
///
/// # Remarks
/// These flags are used when calling [`ID3D11DeviceContext::clear_depth_stencil_view`] the flags
/// can be combined with a bitwise OR.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_CLEAR_FLAG {
    /// Clear the depth buffer, using fast clear if possible, then place the resource in a
    /// compressed state.
    Depth = 0x1,

    /// Clear the stencil buffer, using fast clear if possible, then place the resource in a
    /// compressed state.
    Stencil = 0x2,
}
