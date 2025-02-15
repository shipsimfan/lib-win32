use std::ptr::null_mut;

use crate::{
    dxgi::{DXGI_MODE_DESC, DXGI_SAMPLE_DESC, DXGI_SWAP_EFFECT, DXGI_USAGE},
    BOOL, HWND, UINT,
};

/// Describes a swap chain.
///
/// # Remarks
/// This structure is used by the GetDesc and CreateSwapChain methods.
///
/// In full-screen mode, there is a dedicated front buffer; in windowed mode, the desktop is the front buffer.
///
/// If you create a swap chain with one buffer, specifying DXGI_SWAP_EFFECT_SEQUENTIAL does not cause the contents of the single buffer to be swapped with the front buffer.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_SWAP_CHAIN_DESC {
    /// A DXGI_MODE_DESC structure that describes the backbuffer display mode.
    pub buffer_desc: DXGI_MODE_DESC,

    /// A DXGI_SAMPLE_DESC structure that describes multi-sampling parameters.
    pub sample_desc: DXGI_SAMPLE_DESC,

    /// A member of the DXGI_USAGE enumerated type that describes the surface usage and CPU access options for the back buffer. The back buffer can be used for shader input or render-target output.
    pub buffer_usage: DXGI_USAGE,

    /// A value that describes the number of buffers in the swap chain. When you call IDXGIFactory::CreateSwapChain to create a full-screen swap chain, you typically include the front buffer in this value. For more information about swap-chain buffers, see Remarks.
    pub buffer_count: UINT,

    /// An HWND handle to the output window. This member must not be NULL.
    pub output_window: HWND,

    /// A Boolean value that specifies whether the output is in windowed mode. TRUE if the output is in windowed mode; otherwise, FALSE.
    ///
    /// We recommend that you create a windowed swap chain and allow the end user to change the swap chain to full screen through IDXGISwapChain::SetFullscreenState; that is, do not set this member to FALSE to force the swap chain to be full screen. However, if you create the swap chain as full screen, also provide the end user with a list of supported display modes through the BufferDesc member because a swap chain that is created with an unsupported display mode might cause the display to go black and prevent the end user from seeing anything.
    ///
    /// For more information about choosing windowed verses full screen, see IDXGIFactory::CreateSwapChain.
    pub windowed: BOOL,

    /// A member of the DXGI_SWAP_EFFECT enumerated type that describes options for handling the contents of the presentation buffer after presenting a surface.
    pub swap_effect: DXGI_SWAP_EFFECT,

    /// A member of the DXGI_SWAP_CHAIN_FLAG enumerated type that describes options for swap-chain behavior.
    pub flags: UINT,
}

impl Default for DXGI_SWAP_CHAIN_DESC {
    fn default() -> Self {
        DXGI_SWAP_CHAIN_DESC {
            buffer_desc: DXGI_MODE_DESC::default(),
            sample_desc: DXGI_SAMPLE_DESC::default(),
            buffer_usage: 0,
            buffer_count: 0,
            output_window: null_mut(),
            windowed: 0,
            swap_effect: DXGI_SWAP_EFFECT::Discard,
            flags: 0,
        }
    }
}
