// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGIFactory, DXGI_SWAP_EFFECT},
    dxgi1_2::{IDXGIFactory2, IDXGISwapChain1},
    HWND, WS_EX_LAYOUTRTL,
};

/// Identifies resize behavior when the back-buffer size does not match the size of the target
/// output.
///
/// # Remarks
/// The [`DXGI_SCALING::None`] value is supported only for flip presentation model swap chains that
/// you create with the [`DXGI_SWAP_EFFECT::FlipSequential`] or [`DXGI_SWAP_EFFECT::FlipDiscard`]
/// value. You pass these values in a call to [`IDXGIFactory2::create_swap_chain_for_hwnd`],
/// [`IDXGIFactory2::create_swap_chain_for_core_window`], or
/// [`IDXGIFactory2::create_swap_chain_for_composition`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_SCALING {
    /// Directs DXGI to make the back-buffer contents scale to fit the presentation target size.
    /// This is the implicit behavior of DXGI when you call the [`IDXGIFactory::create_swap_chain`]
    /// method.
    Stretch = 0,

    /// Directs DXGI to make the back-buffer contents appear without any scaling when the
    /// presentation target size is not equal to the back-buffer size. The top edges of the back
    /// buffer and presentation target are aligned together. If the [`WS_EX_LAYOUTRTL`] style is
    /// associated with the [`HWND`] handle to the target output window, the right edges of the
    /// back buffer and presentation target are aligned together; otherwise, the left edges are
    /// aligned together. All target area outside the back buffer is filled with window background
    /// color.
    ///
    /// This value specifies that all target areas outside the back buffer of a swap chain are
    /// filled with the background color that you specify in a call to
    /// [`IDXGISwapChain1::set_background_color`].
    None = 1,

    /// Directs DXGI to make the back-buffer contents scale to fit the presentation target size,
    /// while preserving the aspect ratio of the back-buffer. If the scaled back-buffer does not
    /// fill the presentation area, it will be centered with black borders.
    ///
    /// This constant is supported on Windows Phone 8 and Windows 10.
    ///
    /// Note that with legacy Win32 window swapchains, this works the same as
    /// [`DXGI_SCALING::Stretch`].
    AspectRatioStretch = 2,
}
