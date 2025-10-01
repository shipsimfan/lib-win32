use crate::{
    dxgi::{DXGI_FORMAT, DXGI_SAMPLE_DESC, DXGI_SWAP_EFFECT, DXGI_USAGE},
    dxgi1_2::{DXGI_ALPHA_MODE, DXGI_SCALING},
    BOOL, UINT,
};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIFactory, IDXGISwapChain, DXGI_SWAP_CHAIN_FLAG},
    dxgi1_2::{IDXGIFactory2, IDXGISwapChain1},
    dxgi1_3::IDXGIFactoryMedia,
    FALSE, HWND, TRUE,
};

/// Describes a swap chain.
///
/// # Remarks
/// This structure is used by the [`IDXGIFactory2::create_swap_chain_for_hwnd`],
/// [`IDXGIFactory2::create_swap_chain_for_core_window`],
/// [`IDXGIFactory2::create_swap_chain_for_composition`],
/// [`IDXGIFactoryMedia::create_swap_chain_for_composition_surface_handle`], and
/// [`IDXGISwapChain1::get_desc1`] methods.
///
/// In full-screen mode, there is a dedicated front buffer; in windowed mode, the desktop is the
/// front buffer.
///
/// For a flip-model swap chain (that is, a swap chain that has the
/// [`DXGI_SWAP_EFFECT::FlipDiscard`] or [`DXGI_SWAP_EFFECT::FlipSequential`] value set in the
/// `swap_effect` member), you must set the Format member to [`DXGI_FORMAT::R16G16B16A16Float`],
/// [`DXGI_FORMAT::B8G8R8A8UNorm`], or [`DXGI_FORMAT::R8G8B8A8UNorm`]; you must set the `count`
/// member of the [`DXGI_SAMPLE_DESC`] structure that the SampleDesc member specifies to one and
/// the `quality` member of [`DXGI_SAMPLE_DESC`] to zero because multiple sample antialiasing
/// (MSAA) is not supported; you must set the `buffer_count` member to from two to sixteen.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct DXGI_SWAP_CHAIN_DESC1 {
    /// A value that describes the resolution width. If you specify the width as zero when you call
    /// the [`IDXGIFactory2::create_swap_chain_for_hwnd`] method to create a swap chain, the
    /// runtime obtains the width from the output window and assigns this width value to the
    /// swap-chain description. You can subsequently call the [`IDXGISwapChain1::get_desc1`] method
    /// to retrieve the assigned width value. You cannot specify the width as zero when you call
    /// the [`IDXGIFactory2::create_swap_chain_for_composition`] method.
    pub width: UINT,

    /// A value that describes the resolution height. If you specify the height as zero when you
    /// call the [`IDXGIFactory2::create_swap_chain_for_hwnd`] method to create a swap chain, the
    /// runtime obtains the height from the output window and assigns this height value to the
    /// swap-chain description. You can subsequently call the [`IDXGISwapChain1::get_desc1`] method
    /// to retrieve the assigned height value. You cannot specify the height as zero when you call
    /// the [`IDXGIFactory2::create_swap_chain_for_composition`] method.
    pub height: UINT,

    /// A [`DXGI_FORMAT`] structure that describes the display format.
    pub format: DXGI_FORMAT,

    /// Specifies whether the full-screen display mode or the swap-chain back buffer is stereo.
    /// [`TRUE`] if stereo; otherwise, [`FALSE`]. If you specify stereo, you must also specify a
    /// flip-model swap chain (that is, a swap chain that has the
    /// [`DXGI_SWAP_EFFECT::FlipSequential`] value set in the `swap_effect` member).
    pub stereo: BOOL,

    /// A [`DXGI_SAMPLE_DESC`] structure that describes multi-sampling parameters. This member is
    /// valid only with bit-block transfer (bitblt) model swap chains.
    pub sample_desc: DXGI_SAMPLE_DESC,

    /// A [`DXGI_USAGE`]-typed value that describes the surface usage and CPU access options for
    /// the back buffer. The back buffer can be used for shader input or render-target output.
    pub buffer_usage: DXGI_USAGE,

    /// A value that describes the number of buffers in the swap chain. When you create a
    /// full-screen swap chain, you typically include the front buffer in this value.
    pub buffer_count: UINT,

    /// A [`DXGI_SCALING`]-typed value that identifies resize behavior if the size of the back
    /// buffer is not equal to the target output.
    pub scaling: DXGI_SCALING,

    /// A [`DXGI_SWAP_EFFECT`]-typed value that describes the presentation model that is used by
    /// the swap chain and options for handling the contents of the presentation buffer after
    /// presenting a surface. You must specify the [`DXGI_SWAP_EFFECT::FlipSequential`] value when
    /// you call the [`IDXGIFactory2::create_swap_chain_for_composition`] method because this
    /// method supports only flip presentation model.
    pub swap_effect: DXGI_SWAP_EFFECT,

    /// A [`DXGI_ALPHA_MODE`]-typed value that identifies the transparency behavior of the
    /// swap-chain back buffer.
    pub alpha_mode: DXGI_ALPHA_MODE,

    /// A combination of [`DXGI_SWAP_CHAIN_FLAG`]-typed values that are combined by using a bitwise
    /// OR operation. The resulting value specifies options for swap-chain behavior.
    pub flags: UINT,
}

impl Default for DXGI_SWAP_CHAIN_DESC1 {
    fn default() -> Self {
        DXGI_SWAP_CHAIN_DESC1 {
            width: 0,
            height: 0,
            format: DXGI_FORMAT::Unknown,
            stereo: 0,
            sample_desc: DXGI_SAMPLE_DESC::default(),
            buffer_usage: 0,
            buffer_count: 0,
            scaling: DXGI_SCALING::None,
            swap_effect: DXGI_SWAP_EFFECT::Discard,
            alpha_mode: DXGI_ALPHA_MODE::Unspecified,
            flags: 0,
        }
    }
}
