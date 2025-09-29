// rustdoc imports
#[allow(unused_imports)]
use crate::{
    dxgi::{IDXGISwapChain, DXGI_MODE_DESC},
    dxgi1_2::{DXGI_MODE_DESC1, DXGI_SWAP_CHAIN_FULLSCREEN_DESC},
};

/// Flags indicating how an image is stretched to fit a given monitor's resolution.
///
/// # Remarks
/// Selecting the [`DXGI_MODE_SCALING::Centered`] or [`DXGI_MODE_SCALING::Stretched`] modes can
/// result in a mode change even if you specify the native resolution of the display in the
/// [`DXGI_MODE_DESC`]. If you know the native resolution of the display and want to make sure that
/// you do not initiate a mode change when transitioning a swap chain to full screen (either via
/// `ALT+ENTER` or [`IDXGISwapChain::set_fullscreen_state`]), you should use
/// [`DXGI_MODE_SCALING::Unspecified`].
///
/// This enum is used by the [`DXGI_MODE_DESC1`] and [`DXGI_SWAP_CHAIN_FULLSCREEN_DESC`]
/// structures.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DXGI_MODE_SCALING {
    /// Unspecified scaling.
    Unspecified = 0,

    /// Specifies no scaling. The image is centered on the display. This flag is typically used for
    /// a fixed-dot-pitch display (such as an LED display).
    Centered = 1,

    /// Specifies stretched scaling.
    Stretched = 2,
}
