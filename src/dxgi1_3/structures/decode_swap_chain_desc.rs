use crate::UINT;

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::DXGI_SWAP_CHAIN_FLAG;

/// Used with [`IDXGIFactoryMedia::create_decode_swap_chain_for_composition_surface_handle`] to
/// describe a decode swap chain.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_DECODE_SWAP_CHAIN_DESC {
    /// Can be 0, or a combination of [`DXGI_SWAP_CHAIN_FLAG::FullscreenVideo`] and/or
    /// [`DXGI_SWAP_CHAIN_FLAG::YuvVideo`]. Those named values are members of the
    /// [`DXGI_SWAP_CHAIN_FLAG`] enumerated type, and you can combine them by using a bitwise OR
    /// operation. The resulting value specifies options for decode swap-chain behavior.
    pub flags: UINT,
}

impl Default for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn default() -> Self {
        DXGI_DECODE_SWAP_CHAIN_DESC { flags: 0 }
    }
}
