// rustdoc imports
#[allow(unused_imports)]
use crate::{
    d3d11::{ID3D11DeviceContext, D3D11_MAP},
    DXGI_ERROR_WAS_STILL_DRAWING,
};

/// Specifies how the CPU should respond when an application calls the [`ID3D11DeviceContext::map`]
/// method on a resource that is being used by the GPU.
///
/// # Remarks
/// This enumeration is used by [`ID3D11DeviceContext::map`].
///
/// [`D3D11_MAP_FLAG::DoNotWait`] cannot be used with [`D3D11_MAP::WriteDiscard`] or
/// [`D3D11_MAP::WriteNoOverwrite`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum D3D11_MAP_FLAG {
    /// Specifies that [`ID3D11DeviceContext::map`] should return [`DXGI_ERROR_WAS_STILL_DRAWING`]
    /// when the GPU blocks the CPU from accessing a resource.
    DoNotWait = 0x100000,
}
