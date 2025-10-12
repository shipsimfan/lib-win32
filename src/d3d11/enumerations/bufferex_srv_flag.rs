// rustdoc imports
#[allow(unused_imports)]
use crate::d3d11::D3D11_BUFFEREX_SRV;

/// Identifies how to view a buffer resource.
///
/// # Remarks
/// This enumeration is used by [`D3D11_BUFFEREX_SRV`]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[non_exhaustive]
pub enum D3D11_BUFFEREX_SRV_FLAG {
    /// View the buffer as raw.
    Raw = 0x1,
}
