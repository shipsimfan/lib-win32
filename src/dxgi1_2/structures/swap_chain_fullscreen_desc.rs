use crate::{
    dxgi::{DXGI_MODE_SCALING, DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL},
    BOOL,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{dxgi1_2::IDXGISwapChain1, FALSE, TRUE};

/// Describes full-screen mode for a swap chain.
///
/// # Remarks
/// This structure is used by the [`IDXGIFactory2::create_swap_chain_for_hwnd`] and
/// [`IDXGISwapChain1::get_fullscreen_desc`] methods.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    /// A [`DXGI_RATIONAL`] structure that describes the refresh rate in hertz. Setting the
    /// numerator to 0 forces the native display's refresh rate.
    pub refresh_rate: DXGI_RATIONAL,

    /// A member of the [`DXGI_MODE_SCANLINE_ORDER`] enumerated type that describes the scan-line
    /// drawing mode.
    pub scaline_ordering: DXGI_MODE_SCANLINE_ORDER,

    /// A member of the [`DXGI_MODE_SCALING`] enumerated type that describes the scaling mode.
    pub scaling: DXGI_MODE_SCALING,

    /// A Boolean value that specifies whether the swap chain is in windowed mode. [`TRUE`] if the
    /// swap chain is in windowed mode; otherwise, [`FALSE`].
    pub windowed: BOOL,
}

impl Default for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn default() -> Self {
        DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
            refresh_rate: DXGI_RATIONAL::default(),
            scaline_ordering: DXGI_MODE_SCANLINE_ORDER::Unspecified,
            scaling: DXGI_MODE_SCALING::Unspecified,
            windowed: 0,
        }
    }
}
