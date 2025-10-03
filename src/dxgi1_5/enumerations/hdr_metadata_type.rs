// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi1_5::{IDXGISwapChain4, DXGI_HDR_METADATA_HDR10};

/// Specifies the header metadata type.
///
/// # Remarks
/// This enum is used by the [`IDXGISwapChain4::set_hdr_metadata`] method.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_HDR_METADATA_TYPE {
    /// Indicates there is no header metadata.
    None = 0,

    /// Indicates the header metadata is held by a [`DXGI_HDR_METADATA_HDR10`] structure.
    Hdr10 = 1,

    #[allow(missing_docs)]
    Hdr10Plus = 2,
}
