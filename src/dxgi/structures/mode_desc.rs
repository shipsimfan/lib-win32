use crate::{
    dxgi::{DXGI_FORMAT, DXGI_MODE_SCALING, DXGI_MODE_SCANLINE_ORDER, DXGI_RATIONAL},
    UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::dxgi::{IDXGIFactory, DXGI_SWAP_CHAIN_DESC};

/// Describes a display mode.
///
/// # Remarks
/// This structure is used by the [`IDXGIOutput::get_display_mode_list`] and
/// [`IDXGIOutput::find_closest_matching_mode`] methods.
///
/// The following format values are valid for display modes and when you create a bit-block
/// transfer (bitblt) model swap chain. The valid values depend on the feature level that you are
/// working with.
///  - Feature level >= 9.1
///    - [`DXGI_FORMAT::R8G8B8A8UNorm`]
///    - [`DXGI_FORMAT::R8G8B8A8UNormSRGB`]
///    - [`DXGI_FORMAT::B8G8R8A8UNorm`] (except 10.x on Windows Vista)
///    - [`DXGI_FORMAT::B8G8R8A8UNormSRGB`] (except 10.x on Windows Vista)
///  - Feature level >= 10.0
///    - [`DXGI_FORMAT::R16G16B16A16Float`]
///    - [`DXGI_FORMAT::R10G10B10A2UNorm`]
///  - Feature level >= 11.0
///    - [`DXGI_FORMAT::SR10G10B10XRBiasA2UNorm`]
///
/// You can pass one of these format values to [`ID3D11Device::check_format_support`] to determine
/// if it is a valid format for displaying on screen. If [`ID3D11Device::check_format_support`]
/// returns [`D3D11_FORMAT_SUPPORT_DISPLAY`] in the bit field to which the `format_support`
/// parameter points, the format is valid for displaying on screen.
///
/// Starting with Windows 8 for a flip model swap chain (that is, a swap chain that has the
/// [`DXGI_SWAP_EFFECT::FlipSequential`] value set in the `swap_effect` member of
/// [`DXGI_SWAP_CHAIN_DESC`]), you must set the Format member of [`DXGI_MODE_DESC`] to
/// [`DXGI_FORMAT::R16G16B16A16Float`], [`DXGI_FORMAT::B8G8R8A8UNorm`], or
/// [`DXGI_FORMAT::R8G8B8A8UNorm`].
///
/// Because of the relaxed render target creation rules that Direct3D 11 has for back buffers,
/// applications can create a [`DXGI_FORMAT::B8G8R8A8UNormSRGB`] render target view from a
/// [`DXGI_FORMAT::B8G8R8A8UNorm`] swap chain so they can use automatic color space conversion when
/// they render the swap chain.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_MODE_DESC {
    /// A value that describes the resolution width. If you specify the width as zero when you call
    /// the [`IDXGIFactory::create_swap_chain`] method to create a swap chain, the runtime obtains
    /// the width from the output window and assigns this width value to the swap-chain
    /// description. You can subsequently call the [`IDXGISwapChain::get_desc`] method to retrieve
    /// the assigned width value.
    pub width: UINT,

    /// A value describing the resolution height. If you specify the height as zero when you call
    /// the [`IDXGIFactory::create_swap_chain`] method to create a swap chain, the runtime obtains
    /// the height from the output window and assigns this height value to the swap-chain
    /// description. You can subsequently call the [`IDXGISwapChain::get_desc`] method to retrieve
    /// the assigned height value.
    pub height: UINT,

    /// A [`DXGI_RATIONAL`] structure describing the refresh rate in hertz
    pub refresh_rate: DXGI_RATIONAL,

    /// A [`DXGI_FORMAT`] structure describing the display format.
    pub format: DXGI_FORMAT,

    /// A member of the [`DXGI_MODE_SCANLINE_ORDER`] enumerated type describing the scanline
    /// drawing mode.
    pub scanline_ordering: DXGI_MODE_SCANLINE_ORDER,

    /// A member of the [`DXGI_MODE_SCALING`] enumerated type describing the scaling mode.
    pub scaling: DXGI_MODE_SCALING,
}

impl Default for DXGI_MODE_DESC {
    fn default() -> Self {
        DXGI_MODE_DESC {
            width: 0,
            height: 0,
            refresh_rate: DXGI_RATIONAL::default(),
            format: DXGI_FORMAT::Unknown,
            scanline_ordering: DXGI_MODE_SCANLINE_ORDER::Unspecified,
            scaling: DXGI_MODE_SCALING::Unspecified,
        }
    }
}
